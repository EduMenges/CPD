mod the_algos;

struct MsdSort {
    radix: usize,
    cutoff: usize,
    aux: Vec<String>,
}

impl Default for MsdSort {
    fn default() -> Self {
        Self {
            radix: 256,
            cutoff: 2,
            aux: Default::default(),
        }
    }
}

impl MsdSort {
    pub fn sort(&mut self, src: &mut [String]) {
        self.aux = vec![String::default(); src.len()];
        self.sort_(src, 0, src.len() as isize - 1, 0);
    }

    fn sort_(&mut self, src: &mut [String], lo: usize, hi: isize, pos: usize) {
        if hi <= (lo + self.cutoff) as isize {
            src.sort();
            return;
        }

        let mut count: Vec<usize> = vec![0; self.radix + 2];
        let char_at = |word: &str, n: usize| word.chars().nth(n).unwrap();

        // Computes the frequency count
        for i in lo..=hi as usize {
            count[char_at(&src[i], pos) as usize + 2] += 1;
        }

        // Transform the counts to indices
        for r in 0..self.radix + 1 {
            count[r + 1] += count[r];
        }

        // Distribute
        for i in lo..=hi as usize {
            let index = char_at(&src[i], pos) as usize + 1;
            self.aux[count[index]] = std::mem::replace(&mut src[i], String::default());
            count[index] += 1;
        }

        // Copy back
        for i in lo..=hi as usize {
            src[i] = std::mem::replace(&mut self.aux[i - lo], String::default());
        }

        // Makes the final recursion call
        for r in 0..self.radix {
            self.sort_(
                src,
                lo + count[r],
                (lo + count[r + 1]) as isize - 1,
                pos + 1,
            );
        }
    }
}

pub fn radix_sort(src: &mut [String]) {
    let mut msd_struct = MsdSort::default();
    msd_struct.sort(src);
}
