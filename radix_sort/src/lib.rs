struct MSD {
    radix: usize,
    cutoff: u8,
    aux: Vec<String>,
}

impl Default for MSD {
    fn default() -> Self {
        Self {
            radix: 256,
            cutoff: 2,
            aux: Default::default(),
        }
    }
}

impl MSD {
    pub fn char_at(src: &str, pos: usize) -> isize {
        match src.chars().nth(pos) {
            Some(ch) => ch as isize,
            None => -1,
        }
    }

    pub fn sort(&mut self, src: &mut [String]) {
        self.aux = vec![String::default(); src.len()];
        self.sort_(src, 0);
    }

    fn sort_(&mut self, src: &mut [String], pos: usize) {
        // Cutoff the small subarrays
        if src.len() < self.cutoff as usize {
            src.sort();
            return;
        }

        // Compute the frequency counts
        let mut count = vec![0; self.radix + 2];
        for element in src.as_ref() {
            let index = Self::char_at(element, pos);
            count[(index + 2) as usize] += 1;
        }

        // Perform the accumulation
        for r in 0..=self.radix {
            count[r + 1] += count[r];
        }

        // Move to the aux, sorted
        for i in 0..src.len() {
            let ch = Self::char_at(&src[i], pos);
            let new_i = (ch + 1) as usize;

            std::mem::swap(&mut src[i], &mut self.aux[count[new_i]]);
            count[new_i] += 1;
        }

        // Move back to the src
        for i in 0..src.len() {
            std::mem::swap(&mut src[i], &mut self.aux[i]);
        }

        // Recursively sort for the rest of characters
        for r in 1..self.radix {
            let radix_i = count[r];
            let next_radix = count[r + 1];
            Self::sort_(self, &mut src[radix_i..next_radix], pos + 1);
        }
    }
}

pub fn radix_sort(src: &mut [String]) {
    let mut msd = MSD::default();
    msd.sort(src);
}
