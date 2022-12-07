use quick_sort::{
    partitioning::{Lomuto, Partitioning},
    pivot::*,
    *,
};

fn main() {
    let mut test_arr = [16, 16, 14, 12, 1, 8, 4, 9, 6, 15, 13, 11, 2, 7, 3, 10, 5];

    Lomuto::partition(&mut test_arr);
}
