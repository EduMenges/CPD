pub struct Shell {
    size: usize,
}
pub trait NewCounter<T> {
    fn new(size: usize) -> Self;
}

impl NewCounter<Shell> for Shell {
    fn new(size: usize) -> Shell {
        let exponent = size.ilog2();
        Shell {
            size: 2usize.pow(exponent + 1),
        }
    }
}

impl std::fmt::Display for Shell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SHELL")
    }
}

impl Iterator for Shell {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.size /= 2;

        if self.size > 0 {
            Some(self.size)
        } else {
            None
        }
    }
}

pub struct Knuth {
    size: usize,
}

impl NewCounter<Knuth> for Knuth {
    fn new(size: usize) -> Self {
        let mut gap = 1usize;

        while gap < size {
            gap = gap * 3 + 1;
        }

        Knuth { size: gap }
    }
}

impl std::fmt::Display for Knuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KNUTH")
    }
}

impl Iterator for Knuth {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.size /= 3;

        if self.size > 0 {
            Some(self.size)
        } else {
            None
        }
    }
}

pub struct Ciura {
    sequence: Vec<usize>,
}

impl NewCounter<Ciura> for Ciura {
    fn new(size: usize) -> Self {
        let mut sequence: Vec<usize> = vec![1, 4, 10, 23, 57, 132, 301, 701, 1750];

        if sequence.last().unwrap() > &size {
            while sequence.last().unwrap() > &size {
                sequence.pop();
            }
        } else {
            sequence[8] = 1875;
            while sequence.last().unwrap() < &size {
                sequence.push(sequence.last().unwrap() * 49usize / 20usize);
            }
            sequence[8] = 1750;
            sequence.pop();
        }

        Ciura { sequence }
    }
}

impl std::fmt::Display for Ciura {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CIURA")
    }
}

impl Iterator for Ciura {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.sequence.pop()
    }
}
