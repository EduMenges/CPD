use std::collections::LinkedList;

use shell_sort::{gaps::*, logger::*, *};

pub fn display_steps(parsed_entry: &[u32], log_path: &std::path::Path) {
    shell_sort(
        &mut parsed_entry.to_owned(),
        EachStepLogger::new(log_path, parsed_entry, &Shell::new(1)),
    );
    shell_sort(
        &mut parsed_entry.to_owned(),
        EachStepLogger::new(log_path, parsed_entry, &Knuth::new(1)),
    );
    shell_sort(
        &mut parsed_entry.to_owned(),
        EachStepLogger::new(log_path, parsed_entry, &Ciura::new(1)),
    );
}

pub fn display_steps_for_all(parsed_entries: LinkedList<Vec<u32>>, log_path: &std::path::Path) {
    for parsed_entry in parsed_entries {
        display_steps(&parsed_entry, log_path);
    }
}
