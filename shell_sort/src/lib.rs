pub mod gaps;
pub mod logger;

use std::fmt::Debug;

use gaps::*;
use logger::*;

pub fn shell_sort<T, G, L: Logger<T, G>>(vec: &mut Vec<T>, mut logger: L)
where
    T: std::cmp::PartialOrd<T> + Debug + std::fmt::Display,
    G: NewCounter<G> + std::fmt::Display + Iterator<Item = usize>,
{
    if vec.len() > 1 {
        let gaps = G::new(vec.len());

        logger.log_start();

        for gap in gaps {
            for mut key in gap..vec.len() {
                while (key >= gap) && (vec[key] < vec[key - gap]) {
                    vec.swap(key, key - gap);
                    key -= gap;
                }
            }
            logger.log_step(gap);
        }

        logger.log_end();
    }
}
