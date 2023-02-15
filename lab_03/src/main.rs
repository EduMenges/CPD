use std::io;

use radix_sort::*;

fn main() {
    let mut str_vec: Vec<String> = vec![];

    for line in io::stdin().lines() {
        match line {
            Ok(line) => {
                for word in line.split(' ') {
                    if word.len() >= 4 {
                        str_vec.push(word.to_owned());
                    }
                }
            }
            Err(err) => println!("{err}"),
        }
    }

    println!("Finished processing.");
    println!("{:?}", &str_vec[0..10]);
    radix_sort(&mut str_vec);
    println!("{:?}", &str_vec[0..10]);

}
