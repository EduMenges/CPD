use std::{borrow::Borrow, ops::Deref, path::PathBuf};

use csv::Reader;
use tabled::Tabled;

use crate::{hash_map::HashMap, one_to_many::OneToMany, trie_tree::Trie};

#[derive(Debug, Clone, Copy)]
pub struct Rating {
    pub sofifa_id: u32,
    pub rating: f32,
}

impl OneToMany<u32, Rating> {
    pub fn compute_ratings(&self, players: &mut HashMap<u32, Player>) {
        for ratings in self.iter().map(|(_, user_ratings)| user_ratings) {
            for Rating {
                sofifa_id: fifa_id,
                rating,
            } in ratings
            {
                let player = players.get_mut(&fifa_id).unwrap();

                player.count += 1;
                player.rating += *rating as f64;
            }
        }

        for (_, player) in players.iter_mut().filter(|(_, player)| player.count != 0) {
            player.rating /= player.count as f64;
        }
    }
}

#[derive(Debug, Tabled)]
pub struct Player {
    name: String,
    player_positions: String,
    pub rating: f64,
    pub count: u16,
}

impl Player {
    #[inline(always)]
    pub fn has_position(&self, position: &str) -> bool {
        self.player_positions.contains(position)
    }
}

#[derive(serde::Deserialize, Clone, Copy)]
struct RatingEntry(u32, u32, f32);

pub fn load_ratings(base_path: PathBuf) -> OneToMany<u32, Rating> {
    // let path = base_path.join("minirating.csv");
    let path = base_path.join("rating.csv");

    let mut ratings: OneToMany<u32, Rating> = OneToMany::new(25033);
    let rdr = Reader::from_path(path).unwrap();

    for entry in rdr
        .into_deserialize::<RatingEntry>()
        .map(|rec| rec.unwrap())
    {
        ratings.insert((
            entry.0,
            Rating {
                sofifa_id: entry.1,
                rating: entry.2,
            },
        ));
    }

    ratings
}

#[derive(serde::Deserialize)]
struct TagEntry(u32, u32, String);

pub fn load_tags(base_path: PathBuf) -> OneToMany<String, u32> {
    let path = base_path.join("tags.csv");

    let mut tags = OneToMany::new(199);
    let rdr = Reader::from_path(path).unwrap();

    for entry in rdr.into_deserialize::<TagEntry>().map(|rec| rec.unwrap()) {
        tags.insert((entry.2, entry.1));
    }

    tags
}

#[derive(serde::Deserialize)]
struct PlayerEntry(u32, String, String);

pub fn load_players(base_path: PathBuf, ratings: &OneToMany<u32, Rating>) -> HashMap<u32, Player> {
    let path = base_path.join("players.csv");

    let mut players = HashMap::new(5501);
    let rdr = Reader::from_path(path).unwrap();

    for entry in rdr
        .into_deserialize::<PlayerEntry>()
        .map(|rec| rec.unwrap())
    {
        let fifa_id = entry.0;
        let player = Player {
            name: entry.1,
            player_positions: entry.2,
            rating: 0.0,
            count: 0,
        };
        players.insert((fifa_id, player));
    }

    ratings.compute_ratings(&mut players);

    players
}

pub fn mount_trie(players: &HashMap<u32, Player>) -> Trie<(u32, Player)> {
    let mut trie: Trie<(u32, Player)> = Trie::new();

    for player_and_id in players.iter() {
        trie.insert(&player_and_id.1.name, player_and_id as *const _);
    }

    trie
}
