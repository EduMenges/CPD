use benchmark::BenchmarkData;
use partitioning::Partitioning;
use pivot::Pivot;

pub mod benchmark;
pub mod partitioning;
pub mod pivot;

fn _my_quicksort<T, Pa, Pi, B>(arr: &mut [T], bench: &mut B)
where
    T: std::cmp::Ord + std::fmt::Debug,
    Pa: Partitioning<T, B>,
    Pi: Pivot<T>,
    B: BenchmarkData
{
    bench.increase_recursion();
    if arr.len() > 1 {
        Pi::change_pivot(arr);
        let new_pivot_index = Pa::partition(arr, bench);
        
        _my_quicksort::<T, Pa, Pi, B>(&mut arr[..new_pivot_index], bench);
        _my_quicksort::<T, Pa, Pi, B>(&mut arr[new_pivot_index + 1..], bench);
    }
}

pub fn my_quicksort<T, Pa, Pi, B>(arr: &mut [T]) -> B
where
    T: std::cmp::Ord + std::fmt::Debug,
    Pa: Partitioning<T, B>,
    Pi: Pivot<T>,
    B: BenchmarkData,
{
    let mut bench = B::new();
    
    if arr.len() > 1 {
        _my_quicksort::<T, Pa, Pi, B>(arr, &mut bench);
    }

    bench
}
