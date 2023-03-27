use std::path::PathBuf;

use csv::Reader;

use crate::{hash_map::HashMap, one_to_many::OneToMany};

#[derive(Debug)]
pub struct Rating {
    fifa_id: u32,
    rating: f32,
}

impl OneToMany<u32, Rating> {
    pub fn compute_ratings(&self, players: &mut HashMap<u32, Player>) {
        for user_ratings in self.iter().map(|(_, ratings)| ratings) {
            for Rating { fifa_id, rating } in user_ratings.iter() {
                let player = players.get_mut(fifa_id).unwrap();

                player.count += 1;
                player.rating += *rating as f64;
            }
        }

        for (_, player) in players.iter_mut().filter(|(_, player)| player.count != 0) {
            player.rating /= player.count as f64;
        }
    }
}

#[derive(serde::Deserialize)]
struct RatingEntry(u32, u32, f32);

#[derive(serde::Deserialize)]
struct TagEntry(u32, u32, String);

#[derive(Debug)]
pub struct Player {
    name: String,
    player_positions: String,
    rating: f64,
    count: u16,
}

#[derive(serde::Deserialize)]
struct PlayerEntry(u32, String, String);

pub fn load_ratings(base_path: PathBuf) -> OneToMany<u32, Rating> {
    let path = base_path.join("minirating.csv");

    let mut ratings: OneToMany<u32, Rating> = OneToMany::new(10000);
    let rdr = Reader::from_path(path).unwrap();

    for entry in rdr
        .into_deserialize::<RatingEntry>()
        .map(|rec| rec.unwrap())
    {
        ratings.insert((
            entry.0,
            Rating {
                fifa_id: entry.1,
                rating: entry.2,
            },
        ));
    }

    ratings
}

pub fn load_tags(base_path: PathBuf) -> OneToMany<String, u32> {
    let path = base_path.join("tags.csv");

    let mut tags = OneToMany::new(50);
    let rdr = Reader::from_path(path).unwrap();

    for entry in rdr.into_deserialize::<TagEntry>().map(|rec| rec.unwrap()) {
        tags.insert((entry.2, entry.1));
    }

    tags
}

pub fn load_players(base_path: PathBuf, ratings: &OneToMany<u32, Rating>) -> HashMap<u32, Player> {
    let path = base_path.join("players.csv");

    let mut players = HashMap::new(15000);
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
