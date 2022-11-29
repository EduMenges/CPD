use std::{
    collections::LinkedList,
    fs::File,
    io::Write,
    path,
    time::{Duration, Instant},
};

use shell_sort::{gaps::*, logger::NoLogger, shell_sort};

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
    let duration = benchmark(|| shell_sort::<T, G, _>(line, NoLogger {}));

    let result = format!("{},{},{}", G::new(1), line.len(), duration.as_secs_f64());
    println!("{}", result);
    writeln!(log_file, "{}", result).unwrap();
}

pub fn bench_lines<
    T: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone + std::fmt::Display,
>(
    lines: LinkedList<Vec<T>>,
) {
    let mut log_file = File::create(path::Path::new("./saida2.txt")).unwrap();
    for line in lines {
        bench_line::<T, Shell>(&mut line.clone(), &mut log_file);
        bench_line::<T, Knuth>(&mut line.clone(), &mut log_file);
        bench_line::<T, Ciura>(&mut line.clone(), &mut log_file);
    }
}
