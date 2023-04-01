use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::{
    path::Path,
    time::{Duration, Instant},
};
use tabled::{builder::Builder, *};

use crate::{hash_map::HashMap, load_data::*, one_to_many::OneToMany, trie_tree::Trie};

pub struct DataBase {
    tags: OneToMany<String, u32>,
    ratings: OneToMany<u32, Rating>,
    players: HashMap<u32, Player>,
    trie: Trie<(u32, Player)>,
}

#[derive(Tabled)]
struct PlayerAndId<'a>(
    #[tabled(rename = "sofifa_id")] u32,
    #[tabled(inline)] &'a Player,
);

impl<'a> From<&'a (u32, Player)> for PlayerAndId<'a> {
    fn from(value: &'a (u32, Player)) -> Self {
        Self(value.0, &value.1)
    }
}

impl DataBase {
    pub fn new() -> Self {
        let base_path = Path::new(r".\dados");

        println!("Loading the data...");

        let start_time = Instant::now();

        let ratings = load_ratings(base_path.into());

        let tags = load_tags(base_path.into());

        let players = load_players(base_path.into(), &ratings);

        let end_time = Instant::now();

        let trie = mount_trie(&players);

        println!(
            "The data was loaded. Total time: {:?}",
            end_time.duration_since(start_time)
        );

        Self {
            ratings,
            tags,
            players,
            trie,
        }
    }

    pub fn query_player(&self, prefix: &str) {
        let mut table = self
            .trie
            .search_all(prefix)
            .into_iter()
            .map(|ptr| unsafe {
                let tuple = ptr.as_ref().unwrap();
                PlayerAndId(tuple.0, &tuple.1)
            })
            .table();
        Self::draw_table(table);
    }

    pub fn query_user_players(&self, user_id: u32) {
        #[derive(Tabled)]
        struct RatingAndPlayer<'a>(
            #[tabled(inline)] &'a Player,
            #[tabled(rename = "user_rating")] OrderedFloat<f32>,
        );

        if let Some(ratings) = self.ratings.get(&user_id) {
            let mut table = ratings
                .iter()
                .map(|Rating { sofifa_id, rating }| {
                    let player = self.players.get(sofifa_id).unwrap();
                    RatingAndPlayer(player, OrderedFloat(*rating))
                })
                .sorted_unstable_by_key(|RatingAndPlayer(_, rating)| *rating)
                .rev()
                .take(20)
                .table();
            Self::draw_table(table);
        }
    }

    pub fn query_best_in_position(&self, max_amount: usize, position: &str) {
        const MIN_RATING_COUNT: u16 = 1000;

        let mut table = self
            .players
            .iter()
            .filter(|(_, player)| player.count > MIN_RATING_COUNT && player.has_position(position))
            .sorted_unstable_by_key(|(_, player)| OrderedFloat(player.rating))
            .rev()
            .take(max_amount)
            .map(|(id, player)| PlayerAndId(*id, player))
            .table();
        Self::draw_table(table);
    }

    pub fn query_tags(&self, tags: &[&str]) {
        let mut table = self
            .players
            .iter()
            .filter(|(id, _)| tags.iter().all(|tag| self.tag_has_player(tag, *id)))
            .map(|entry| PlayerAndId::from(entry))
            .table();
        Self::draw_table(table);
    }

    fn tag_has_player(&self, tag: &str, id: u32) -> bool {
        match self.tags.get(&String::from(tag)) {
            Some(ids) => ids.contains(&id),
            None => false,
        }
    }

    fn draw_table(mut table: Table) {
        let table = table.with(Style::modern());
        println!("{table}");
    }
}
