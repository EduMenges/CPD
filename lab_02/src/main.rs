use quick_sort::{partitioner::*, *};

fn main() {
    let test_arr = [16, 16, 14, 12, 1, 8, 4, 9, 6, 15, 13, 11, 2, 7, 3, 10, 5];

    let median_part = MedianOf3::get_partitioner(&test_arr);
    let random_part = RandomPartition::get_partitioner(&test_arr);

    println!("{:?} {:?}", median_part, random_part);
}
