use std::{
    path::Path,
    time::{Duration, Instant},
};

use crate::{load_data::*, one_to_many::OneToMany};

pub struct DataBase {
    tags: OneToMany<String, u32>,
    ratings: OneToMany<u32, Rating>,
}

impl DataBase {
    pub fn new() -> Self {
        let base_path = Path::new(r".\dados");

        println!("Loading the data...");

        let start_time = Instant::now();

        let ratings = load_ratings(base_path.into());

        let tags = load_tags(base_path.into());

        let end_time = Instant::now();

        println!(
            "The data was loaded. Total time: {:?}",
            end_time.duration_since(start_time)
        );

        Self { ratings, tags }
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
}
