use std::{
    collections::LinkedList,
    fs::File,
    io::Write,
    time::{Duration, Instant},
};

use shell_sort::{gaps::*, shell_sort};

pub fn benchmark<F>(closure: F) -> Duration
where
    F: FnOnce(),
{
    let earlier = Instant::now();
    closure();

    earlier.elapsed()
}

pub fn bench_line<T, G>(line: &mut Vec<T>, log_file: &mut File)
where
    T: std::fmt::Debug + std::cmp::PartialOrd + std::clone::Clone + std::fmt::Display,
    G: shell_sort::gaps::NewCounter<G> + std::fmt::Display + Iterator<Item = usize>,
{
    let mut dummy = File::create(".temp").unwrap();
    let duration = benchmark(|| shell_sort::<T, G>(line, false, &mut dummy));

    let result = format!("{},{},{}", G::new(1), line.len(), duration.as_secs_f64());
    println!("{}", result);
    writeln!(log_file, "{}", result).unwrap();
}

pub fn bench_lines<
    T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone + std::fmt::Display,
>(
    lines: LinkedList<Vec<T>>,
    log_file: &mut File,
) {
    for line in lines {
        bench_line::<T, Shell>(&mut line.clone(), log_file);
        bench_line::<T, Knuth>(&mut line.clone(), log_file);
        bench_line::<T, Ciura>(&mut line.clone(), log_file);
    }
}
