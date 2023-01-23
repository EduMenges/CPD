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
            cutoff: 15,
            aux: Default::default(),
        }
    }
}

impl MsdSort {
    pub fn sort(&mut self, src: &mut [String]) {
        self.aux = Vec::with_capacity(src.len());
        self.sort_(src, 0, src.len() - 1, 0);
    }

    fn sort_(&mut self, src: &mut [String], lo: usize, hi: usize, pos: usize) {
        // if hi <= lo + self.cutoff {
        //     src.sort();
        //     return;
        // }

        let mut count: Vec<usize> = vec![0; self.radix + 2];
        let char_at = |word: &str, n: usize| word.chars().nth(n).unwrap();

        // Computes the frequency count
        for i in lo..=hi {
            count[char_at(&src[i], pos) as usize + 2] += 1;
        }

        // Transform the counts to indices
        for r in 1..count.len() {
            count[r] += count[r - 1];
        }

        // Distribute
        for i in lo..=hi {
            let index = char_at(&src[i], pos) as usize + 1;
            count[index] += 1;
            self.aux[index] = std::mem::replace(&mut src[i], String::default());
        }

        // Copy back
        for i in lo..=hi {
            src[i] = std::mem::replace(&mut self.aux[i - lo], String::default());
        }

        // Makes the final recursion call
        for r in 0..self.radix {
            self.sort_(src, lo + count[r], lo + count[r + 1] - 1, pos + 1);
        }
    }
}

pub fn radix_sort(src: &mut [String]) {
    let mut msd_struct = MsdSort::default();
    msd_struct.sort(src);
}
