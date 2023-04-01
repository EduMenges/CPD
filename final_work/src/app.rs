use std::{
    path::Path,
    time::{Duration, Instant},
};

use crate::{hash_map::HashMap, load_data::*, one_to_many::OneToMany};

pub struct DataBase {
    tags: OneToMany<String, u32>,
    ratings: OneToMany<u32, Rating>,
    players: HashMap<u32, Player>,
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

        println!(
            "The data was loaded. Total time: {:?}",
            end_time.duration_since(start_time)
        );

        Self {
            ratings,
            tags,
            players,
        }
    }
    pub fn print_ratings(&self) {
        for rating in self.ratings.iter().take(50) {
            println!("{:?}", rating);
        }
    }

    pub fn print_tags(&self) {
        for tag in self.tags.iter().take(50) {
            println!("{:?}", tag);
        }
    }

    pub fn print_players(&self) {
        for player in self.players.iter().take(50) {
            println!("{:?}", player);
        }
    }
}
