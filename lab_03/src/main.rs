use std::{collections::BTreeMap, io};

use radix_sort::*;

fn main() {
    let mut str_vec: Vec<String> = Vec::new();

    for line in io::stdin().lines() {
        match line {
            Ok(line) => {
                for word in line.split_whitespace() {
                    if word.len() >= 4 {
                        str_vec.push(word.to_owned());
                    }
                }
            }
            Err(err) => println!("{err}"),
        }
    }

    radix_sort(&mut str_vec);

    let mut frequency_map: BTreeMap<&str, u16> = BTreeMap::new();
    str_vec.iter().for_each(|key| {
        frequency_map
            .entry(&key)
            .and_modify(|acc| *acc += 1)
            .or_insert(1);
    });

    for (key, val) in frequency_map {
        println!("{key} {val}");
    }
}
