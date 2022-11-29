use std::{collections::LinkedList, fs::File};

use shell_sort::{gaps::*, logger::*, *};

pub fn display_steps(parsed_entry: &[u32], common_file: &mut File) {
    shell_sort(
        &mut parsed_entry.to_owned(),
        EachStepLogger{ gap: &Shell::new(1), file: common_file, vec: parsed_entry }
    );
    shell_sort(
        &mut parsed_entry.to_owned(),
        EachStepLogger{ gap: &Knuth::new(1), file: common_file, vec: parsed_entry },
    );
    shell_sort(
        &mut parsed_entry.to_owned(),
        EachStepLogger{ gap: &Ciura::new(1), file: common_file, vec: parsed_entry },
    );
}

pub fn display_steps_for_all(parsed_entries: LinkedList<Vec<u32>>, log_path: &std::path::Path) {
    let mut common_file = File::create(log_path).unwrap();
    
    for parsed_entry in parsed_entries {
        display_steps(&parsed_entry, &mut common_file);
    }
}
