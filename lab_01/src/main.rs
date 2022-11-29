mod benchmark;
pub mod display_steps;

use std::path;

use benchmark::bench_lines;
use display_steps::display_steps_for_all;
use file_parser::file_parser;

fn main() {
    let entrada1_path = path::Path::new("./entradas/entrada1.txt");
    let entrada2_path = path::Path::new("./entradas/entrada2.txt");

    let entrada1_parsed = file_parser(entrada1_path);
    let entrada2_parsed = file_parser(entrada2_path);

    display_steps_for_all(entrada1_parsed, path::Path::new("./saida1.txt"));
    bench_lines(entrada2_parsed);
}
