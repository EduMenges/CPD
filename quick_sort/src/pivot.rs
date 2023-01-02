use rand::{thread_rng, Rng};

pub trait Pivot<T: std::cmp::Ord> {
    fn get_pivot(arr: &[T]) -> usize;
    fn change_pivot(arr: &mut [T]) {
        arr.swap(Self::get_pivot(arr), 0);
    }
}

pub struct MedianOf3 {}
impl<T: std::cmp::Ord> Pivot<T> for MedianOf3 {
    fn get_pivot(arr: &[T]) -> usize {
        let first = arr.first().unwrap();
        let last = arr.last().unwrap();

        let medium_index = arr.len() / 2;
        let last_index = arr.len() - 1;

        let medium = arr.get(medium_index).unwrap();

        let mut median = [first, last, medium];
        median.sort();

        if median[1] == first {
            0
        } else if median[1] == medium {
            medium_index
        } else {
            last_index
        }
    }
}

pub struct RandomPartition {}
impl<T: std::cmp::Ord> Pivot<T> for RandomPartition {
    fn get_pivot(arr: &[T]) -> usize {
        let mut rng = thread_rng();
        rng.gen_range(0..arr.len())
    }
}
