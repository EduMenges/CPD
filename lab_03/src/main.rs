use std::io;

use radix_sort::*;

fn main() {
    let mut str_vec: Vec<String> = vec![];
    // let mut test: Vec<String> = vec![
    //     "dab".to_string(),
    //     "add".to_string(),
    //     "cab".to_string(),
    //     "fad".to_string(),
    //     "fee".to_string(),
    //     "bad".to_string(),
    //     "dad".to_string(),
    //     "bee".to_string(),
    //     "fed".to_string(),
    //     "bed".to_string(),
    //     "ebb".to_string(),
    //     "ace".to_string(),
    // ];

    for line in io::stdin().lines() {
        match line {
            Ok(line) => {
                if line.len() >= 4 {
                    str_vec.push(line);
                }
            }
            Err(err) => println!("{err}"),
        }
    }

    radix_sort(&mut str_vec);

    println!("{:?}", &str_vec[0..10]);
}
