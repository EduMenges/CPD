use quick_sort::{
    benchmark::benchmark,
    partitioning::{Lomuto, Partitioning},
    pivot::*,
    *,
};

fn main() {
    let mut vec_test = vec![1, 2, 3];
    let mut test_arr = [
        100, 46, 18, 26, 1, 57, 65, 98, 49, 5, 51, 51, 77, 13, 55, 98, 5, 82, 49, 66, 79, 100, 10,
        30, 54, 84, 48, 8, 74, 23, 6, 77, 54, 25, 31, 67, 1, 45, 71, 86, 17, 99, 56, 34, 58, 77,
        79, 67, 58, 12, 79, 15, 20, 38, 34, 92, 21, 63, 34, 98, 32, 89, 90, 74, 78, 5, 59, 1, 55,
        61, 16, 48, 81, 31, 56, 47, 8, 39, 1, 27, 36, 58, 14, 15, 94, 61, 55, 19, 58, 86, 30, 16,
        64, 3, 18, 44, 20, 68, 48, 70, 96,
    ];

    println!("{:?}", test_arr);

    let result = benchmark(|| {
        my_quicksort::<_, partitioning::Hoare, pivot::MedianOf3, benchmark::NormalBenchmark>(
            &mut test_arr,
        )
    });

    println!("{}", result);

    println!("{:?}", test_arr);
}
