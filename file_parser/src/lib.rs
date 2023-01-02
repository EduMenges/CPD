use std::collections::LinkedList;
use std::io::BufRead;
use std::{fs, io};

pub fn file_parser(path_: &std::path::Path) -> LinkedList<Vec<u32>> {
    let mut ret: LinkedList<Vec<u32>> = LinkedList::new();

    let file = fs::File::open(path_).unwrap();
    let file_lines = io::BufReader::new(file).lines();

    for line in file_lines.flatten() {
        ret.push_back(line_parser(&line));
    }

    ret
}

pub fn line_parser(line: &str) -> Vec<u32> {
    let string_and_n = line.split_once(' ');

    if let Some((element_amount, line)) = string_and_n {
        let mut ret: Vec<u32> = Vec::with_capacity(element_amount.parse().unwrap());

        line.split(' ').for_each(|n| {
            if let Ok(n) = n.parse() {
                ret.push(n);
            }
        });

        ret
    } else {
        Vec::new()
    }
}
