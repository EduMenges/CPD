pub trait Partitioning<T> {
    fn partition(arr: &mut [T]);
}

pub struct Lomuto {}
impl<T: std::cmp::PartialOrd> Partitioning<T> for Lomuto {
    fn partition(arr: &mut [T]) {
        let mut store_index: usize = 1; // Index of the oldest smaller element

        for i in 1..arr.len() {
            if arr[i] < arr[0] {
                arr.swap(store_index, i);
                store_index += 1;
            }
        }

        arr.swap(0, store_index - 1);
    }
}
