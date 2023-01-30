use std::io;

use radix_sort::*;

fn main() {
    // let mut str_vec: Vec<String> = Vec::new();
    //
    // for line in io::stdin().lines() {
    //     match line {
    //         Ok(line) => {
    //             for word in line.split_whitespace() {
    //                 if word.len() >= 4 {
    //                     str_vec.push(word.to_owned());
    //                 }
    //             }
    //         }
    //         Err(err) => println!("{err}"),
    //     }
    // }

    let mut str_vec: Vec<String> = vec![
        "dab".to_owned(),
        "addb".to_owned(),
        "cabcf".to_owned(),
        "fadde".to_owned(),
        "feee".to_owned(),
        "badf".to_owned(),
        "adb".to_owned(),
        "dada".to_owned(),
        "beeba".to_owned(),
        "fedc".to_owned(),
        "bedd".to_owned(),
        "ebb".to_owned(),
        "acef".to_owned(),
        "baee".to_owned(),
        "baeb".to_owned(),
    ];

    radix_sort(&mut str_vec);

    println!("{:?}", &str_vec);
}
