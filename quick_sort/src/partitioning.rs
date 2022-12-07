pub trait Partitioning<T> {
    fn partition(arr: &mut [T]);
}

pub struct Lomuto {}
impl<T> Partitioning<T> for Lomuto {
    fn partition(arr: &mut [T]) {
        let key = arr.first.unwrap();
    }
}
