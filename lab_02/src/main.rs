use std::io;

use file_parser::line_parser;
use quick_sort::{
    benchmark::benchmark,
    partitioning::{Lomuto, Partitioning},
    pivot::*,
    *,
};

fn main() {
    for line in io::stdin().lines() {
        match line {
            Ok(line) => {
                let mut test_arr = line_parser(&line);
                let result = benchmark(|| {
                    my_quicksort::<
                        _,
                        partitioning::Hoare,
                        pivot::RandomPartition,
                        benchmark::NormalBenchmark,
                    >(&mut test_arr)
                });
                println!("TAMANHO ENTRADA {}", test_arr.len());
                println!("{result}");
            }
            Err(error) => println!("Error while parsing line: {error}"),
        }
    }
}
