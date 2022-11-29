use std::io::Write;

pub trait Logger<T, G> {
    fn log_step(&mut self, step: usize);
    fn log_start(&mut self);
    fn log_end(&mut self);
}

pub struct NoLogger {}
impl<T, G> Logger<T, G> for NoLogger {
    fn log_step(&mut self, _: usize) {}
    fn log_start(&mut self) {}
    fn log_end(&mut self) {}
}

pub struct EachStepLogger<'a, T, G> {
    gap: &'a G,
    file: std::fs::File,
    vec: &'a [T],
}
impl<'a, T, G> EachStepLogger<'a, T, G>
where
    T: std::fmt::Display,
    G: std::fmt::Display,
{
    pub fn new(file_path: &std::path::Path, vec: &'a [T], gap: &'a G) -> Self {
        EachStepLogger {
            gap,
            file: std::fs::File::create(file_path).unwrap(),
            vec,
        }
    }
}
impl<T, G> Logger<T, G> for EachStepLogger<'_, T, G>
where
    T: std::fmt::Display,
    G: std::fmt::Display,
{
    fn log_step(&mut self, step: usize) {
        let log_buffer = format!(
            "{}INCR={step}",
            self.vec
                .iter()
                .fold(String::new(), |acc, num| acc + &num.to_string() + ", ")
        );
        writeln!(self.file, "{}", log_buffer).unwrap();
        println!("{}", log_buffer);
    }

    fn log_start(&mut self) {
        let log_buffer = format!(
            "{}SEQ={}",
            self.vec
                .iter()
                .fold(String::new(), |acc, num| acc + &num.to_string() + ", "),
            self.gap
        );
        writeln!(self.file, "{}", log_buffer).unwrap();
        println!("{}", log_buffer);
    }

    fn log_end(&mut self) {}
}
