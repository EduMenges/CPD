use partitioner::Partitioner;
use partitioning::Partitioning;

pub mod partitioner;
pub mod partitioning;

struct QuickSort<T> {
    partitioner: dyn Partitioner<T>,
    partitioning: dyn Partitioning<T>,
}
impl<T> QuickSort<T> {
    pub fn new(partitioner: dyn Partitioner<T>, partitioning: dyn Partitioning<T>) {
        QuickSort {
            partitioner,
            partitioning,
        }
    }
}
