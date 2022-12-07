use rand::{seq::SliceRandom, thread_rng};

pub trait Pivot<T: std::cmp::Ord> {
    fn get_pivot(arr: &[T]) -> &T;
}

pub struct MedianOf3 {}
impl<T: std::cmp::Ord> Pivot<T> for MedianOf3 {
    fn get_pivot(arr: &[T]) -> &T {
        let first = arr.first().unwrap();
        let last = arr.last().unwrap();
        let medium = arr.get(arr.len() / 2 - 1).unwrap();

        let mut median = [first, last, medium];
        median.sort();
        median.get(1).unwrap()
    }
}

pub struct RandomPartition {}
impl<T: std::cmp::Ord> Pivot<T> for RandomPartition {
    fn get_pivot(arr: &[T]) -> &T {
        let mut rng = thread_rng();
        arr.choose(&mut rng).unwrap()
    }
}
