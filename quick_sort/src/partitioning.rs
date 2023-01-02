use crate::benchmark::BenchmarkData;

pub trait Partitioning<T, B>
where B: BenchmarkData {
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
        let mut i = 0;
        let mut j = arr.len() - 1;

        if arr[j] == arr[pivot] {
            i += 1;
        }

        loop {
            // Finds the first left element larger than the pivot
            while arr[i] < arr[pivot] {
                i += 1;
            }
            // Finds the first right element smaller than the pivot
            while arr[j] > arr[pivot] {
                j -= 1;
            }
            // In this case, we have transversed through the whole array
            if i < j {
                // If we are in a duplicated element, i needs to advance forward
                if arr[i] == arr[pivot] && arr[j] == arr[pivot] {
                    i += 1;
                } else {
                    arr.swap(i, j);
                    bench.increase_swap();
                }
            } else {
                return i;
            }
        }
    }
}
