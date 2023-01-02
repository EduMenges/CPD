use std::{sync::Mutex, time::{Duration, Instant}, fmt::Display};

pub trait BenchmarkData {
    fn new() -> Self;
    fn increase_swap(&mut self);
    fn increase_recursion(&mut self);
    fn set_time(&mut self, time: Duration);
}

pub struct NoBenchmark;
impl BenchmarkData for NoBenchmark {
    fn new() -> Self {
        Self
    }

    fn increase_swap(&mut self) {}

    fn increase_recursion(&mut self) {}

    fn set_time(&mut self, _: Duration) {}
}

pub struct NormalBenchmark {
    recursion: Mutex<u32>,
    swap: Mutex<u32>,
    time: Duration,
}
impl BenchmarkData for NormalBenchmark {
    fn new() -> Self {
        Self {
            swap: Mutex::new(0),
            recursion: Mutex::new(0),
            time: Default::default(),
        }
    }

    fn set_time(&mut self, time: Duration) {
        self.time = time;
    }

    fn increase_swap(&mut self) {
        *self.swap.lock().unwrap() += 1;
    }

    fn increase_recursion(&mut self) {
        *self.recursion.lock().unwrap() += 1;
    }
}

pub fn benchmark<F, B>(closure: F) -> B
where
    B: BenchmarkData,
    F: FnOnce() -> B,
{
    let earlier = Instant::now();
    let mut result = closure();
    result.set_time(earlier.elapsed());
    result
}

impl Display for NormalBenchmark{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SWAPS {}\nRECURSOES {}\nTEMPO {}", self.swap.lock().unwrap(), self.recursion.lock().unwrap(), self.time.as_secs_f64())
    }
}