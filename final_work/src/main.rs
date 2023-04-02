#![feature(let_chains)]
#![feature(exact_size_is_empty)]

use std::collections::LinkedList;

use app::DataBase;

mod app;
mod hash;
mod hash_map;
mod load_data;
mod one_to_many;
mod parser;
mod trie_tree;

fn main() {
    let db = DataBase::new();
    db.main_loop()
}
