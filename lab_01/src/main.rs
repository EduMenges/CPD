mod benchmark;
pub mod display_steps;

use std::{fs::File, path};

use benchmark::bench_lines;
use display_steps::display_steps_for_all;
use file_parser::file_parser;

fn main() {
    let entrada1_path = path::Path::new("./entradas/entrada1.txt");
    let entrada2_path = path::Path::new("./entradas/entrada2.txt");

    let entrada1_parsed = file_parser(entrada1_path);
    let entrada2_parsed = file_parser(entrada2_path);

    let mut log_file = File::create("./saida1.txt").unwrap();

    display_steps_for_all(entrada1_parsed, &mut log_file);

    let mut log_file = File::create("./saida2.txt").unwrap();

    bench_lines(entrada2_parsed, &mut log_file);
}
