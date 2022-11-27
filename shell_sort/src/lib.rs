pub mod gaps;

use std::{fmt::{Debug}, fs::File, io::Write};

use gaps::*;

pub fn shell_sort<T, G>(vec: &mut Vec<T>, display_steps: bool, log_file: &mut File)
where
    T: std::cmp::PartialOrd<T> + Debug + std::fmt::Display,
    G: NewCounter<G> + std::fmt::Display + Iterator<Item = usize>,
{
    let mut log_buffer: String;

    if vec.len() > 1 {
        let gaps = G::new(vec.len());

        if display_steps {
            log_buffer = format!("{}SEQ={}", vec.iter().fold(String::new(), |acc, num| acc + &num.to_string() + ", "), gaps);
            writeln!(log_file, "{}", log_buffer).unwrap();
            println!("{}", log_buffer);
        }

        for gap in gaps {
            for mut key in gap..vec.len() {
                while (key >= gap) && (vec[key] < vec[key - gap]) {
                    vec.swap(key, key - gap);
                    key -= gap;
                }
            }
            if display_steps {
                log_buffer = format!("{}INCR={gap}", vec.iter().fold(String::new(), |acc, num| acc + &num.to_string() + ", "));
                writeln!(log_file, "{}", log_buffer).unwrap();
                println!("{}", log_buffer);
            }
        }
    }
}
