use std::path::PathBuf;

use csv::Reader;

use crate::{hash_map::HashMap, one_to_many::OneToMany};

#[derive(serde::Deserialize)]
pub struct Rating {
    fifa_id: u32,
    rating: f32,
}

#[derive(serde::Deserialize)]
struct RatingEntry(u32, Rating);

#[derive(serde::Deserialize)]
struct Player {
    name: String,
    player_positions: Vec<String>,
    rating: f64,
    count: u16,
}

pub fn load_ratings(base_path: PathBuf) -> OneToMany<u32, Rating> {
    let path = base_path.join("minirating.csv");

    let mut ratings = OneToMany::new(10000);
    let rdr = Reader::from_path(path).unwrap();

    for record in rdr.into_records() {
        let entry: RatingEntry = record.unwrap().deserialize(None).unwrap();
        ratings.insert((entry.0, entry.1));
    }

    ratings
}
