use std::path::Path;

use crate::{load_data::*, one_to_many::OneToMany};

pub struct DataBase {
    ratings: OneToMany<u32, Rating>,
}

impl DataBase {
    pub fn new() -> Self {
        let base_path = Path::new(r".\dados");

        let ratings = load_ratings(base_path.into());
        Self { ratings }
    }
}
