use std::{fs::File, collections::LinkedList};

use shell_sort::{*, gaps::*};

pub fn display_steps(parsed_entry: &[u32], log_file: &mut File) {
    shell_sort::<u32, Shell>(&mut parsed_entry.to_owned(), true, log_file);
    shell_sort::<u32, Knuth>(&mut parsed_entry.to_owned(), true, log_file);
    shell_sort::<u32, Ciura>(&mut parsed_entry.to_owned(), true, log_file);
}

pub fn display_steps_for_all(parsed_entries: LinkedList<Vec<u32>>, log_file: &mut File) {
    for parsed_entry in parsed_entries {
        display_steps(&parsed_entry, log_file);
    }
}
