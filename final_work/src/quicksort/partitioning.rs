use crate::benchmark::BenchmarkData;

pub trait Partitioning<T, B>
where
    B: BenchmarkData,
{
    fn partition(arr: &mut [T], bench: &mut B) -> usize;
}

pub struct Lomuto;
impl<T: std::cmp::PartialOrd, B: BenchmarkData> Partitioning<T, B> for Lomuto {
    fn partition(arr: &mut [T], bench: &mut B) -> usize {
        let mut store_index: usize = 1; // Index of the oldest largest element

        for i in 1..arr.len() {
            if arr[i] < arr[0] {
                arr.swap(store_index, i);
                bench.increase_swap();
                store_index += 1;
            }
        }

        arr.swap(0, store_index - 1);
        store_index - 1
    }
}

pub struct Hoare;
impl<T: std::cmp::PartialOrd, B: BenchmarkData> Partitioning<T, B> for Hoare {
    fn partition(arr: &mut [T], bench: &mut B) -> usize {
        let pivot = 0_usize;
        let mut i = -1_isize;
        let mut j = arr.len();

        loop {
            while {
                i += 1;
                arr[i as usize] < arr[pivot]
            } {}

            while {
                j -= 1;
                arr[j] > arr[pivot]
            } {}

            if i as usize >= j {
                break;
            } else {
                arr.swap(i as usize, j);
                bench.increase_swap();
            }
        }
        arr.swap(i as usize, pivot);
        bench.increase_swap();
        i as usize
    }
}
