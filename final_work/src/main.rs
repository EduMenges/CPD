#![feature(let_chains)]
#![feature(exact_size_is_empty)]

use std::collections::LinkedList;

use app::DataBase;

mod app;
mod hash;
mod hash_map;
mod load_data;
mod one_to_many;
mod trie_tree;

fn main() {
    let db = DataBase::new();
    // db.query_player("Fer");
    // db.query_user_players(118046);
    // db.query_best_in_position(10, "ST");
    let query = ["Brazil", "Dribler"];
    db.query_tags(&query);
}
