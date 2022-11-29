use std::io::Write;

pub trait Logger<'a, T, G> {
    fn log_step(&mut self, step: usize, vec: &[T]);
    fn log_start(&mut self, vec: &[T]);
    fn log_end(&mut self);
}

pub struct NoLogger {}
impl<T, G> Logger<'_, T, G> for NoLogger {
    fn log_step(&mut self, _: usize, _: &[T]) {}
    fn log_start(&mut self, _: &[T]) {}
    fn log_end(&mut self) {}
}

pub struct EachStepLogger<'a, G> {
    pub gap: &'a G,
    pub file: &'a mut std::fs::File,
}
impl<'a, T, G> Logger<'a, T, G> for EachStepLogger<'a, G>
where
    T: std::fmt::Display,
    G: std::fmt::Display,
{
    fn log_step(&mut self, step: usize, vec: &[T]) {
        let log_buffer = format!(
            "{}INCR={step}",
            vec.iter()
                .fold(String::new(), |acc, num| acc + &num.to_string() + " ")
        );
        writeln!(self.file, "{}", log_buffer).unwrap();
        println!("{}", log_buffer);
    }

    fn log_start(&mut self, vec: &[T]) {
        let log_buffer = format!(
            "{}SEQ={}",
            vec.iter()
                .fold(String::new(), |acc, num| acc + &num.to_string() + " "),
            self.gap
        );
        writeln!(self.file, "{}", log_buffer).unwrap();
        println!("{}", log_buffer);
    }

    fn log_end(&mut self) {}
}
