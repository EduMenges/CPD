use std::{
    io::{stdin, stdout, Write},
    path::Path,
    thread,
    time::Instant,
};

use itertools::Itertools;
use ordered_float::OrderedFloat;
use tabled::*;

use crate::{hash_map::HashMap, load_data::*, one_to_many::OneToMany, parser, trie_tree::Trie};

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

impl OneToMany<String, u32> {
    pub fn has_id(&self, key: &str, id: u32) -> bool {
        match self.get(key) {
            Some(ids) => ids.contains(&id),
            None => false,
        }
    }
}

impl DataBase {
    pub fn new() -> Self {
        let base_path = Path::new(r"./dados");

        println!("Loading the data...");

        let start_time = Instant::now();

        let ratings_handle = thread::spawn(|| load_ratings(base_path.into()));

        let players_handle = thread::spawn(|| load_players(base_path.into()));

        let tags = load_tags(base_path.into());

        let mut players = players_handle.join().unwrap();

        let ratings = ratings_handle.join().unwrap();

        ratings.compute_ratings(&mut players);

        let trie = mount_trie(&players);

        let end_time = Instant::now();

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

    pub fn main_loop(&self) {
        loop {
            let mut input = String::new();

            print!("> ");
            let _ = stdout().flush();

            match stdin().read_line(&mut input) {
                Ok(_) => {
                    match parser::parse(&input) {
                        Some(action) => match action {
                            parser::Actions::Player(name) => self.query_player(&name),
                            parser::Actions::User(id) => self.query_user_players(id),
                            parser::Actions::Top(max_amount, position) => {
                                self.query_best_in_position(max_amount, &position)
                            }
                            parser::Actions::Tags(tags) => self.query_tags(&tags),
                            parser::Actions::Quit => std::process::exit(0)
                        },
                        _ => println!("The input could not be parsed"),
                    };
                    println!();
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    pub fn query_player(&self, prefix: &str) {
        let table = self
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
            let table = ratings
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
        } else {
            println!("The user could not be found in the database");
        }
    }

    pub fn query_best_in_position(&self, max_amount: usize, position: &str) {
        const MIN_RATING_COUNT: u16 = 1000;

        Self::draw_table(
            self.players
                .iter()
                .filter(|(_, player)| {
                    player.count > MIN_RATING_COUNT && player.has_position(position)
                })
                .sorted_unstable_by_key(|(_, player)| OrderedFloat(player.rating))
                .rev()
                .take(max_amount)
                .map(|(id, player)| PlayerAndId(*id, player))
                .table(),
        );
    }

    pub fn query_tags<S: AsRef<str>>(&self, tags: &[S]) {
        let table = self
            .players
            .iter()
            .filter(|(id, _)| tags.iter().all(|tag| self.tags.has_id(tag.as_ref(), *id)))
            .map(|entry| PlayerAndId::from(entry))
            .table();
        Self::draw_table(table);
    }

    fn draw_table(mut table: Table) {
        let table = table.with(Style::modern());
        if table.count_rows() == 1 {
            println!("No results were found for this query.");
        } else {
            println!("{table}");
        }
    }
}
