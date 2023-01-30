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
    pub fn char_at(src: &str, pos: usize) -> usize {
        match src.chars().nth(pos) {
            Some(ch) => ch as usize,
            None => 0,
        }
    }

    pub fn sort(&mut self, src: &mut [String]) {
        self.aux = Vec::with_capacity(src.len());
        self.sort_(src, 0);
    }

    fn sort_(&mut self, src: &mut [String], pos: usize) {
        // Cutoff the small subarrays
        if src.len() < self.cutoff as usize {
            src.sort();
            return;
        }

        // Compute the frequency counts
        let mut count = vec![0; self.radix];
        for element in src.as_ref() {
            let index = Self::char_at(element, pos);
            count[index] += 1;
        }

        // Perform the accumulation
        let mut accumulation = vec![0; self.radix];
        for i in 1..accumulation.len() {
            accumulation[i] += accumulation[i - 1];
        }

        // Move to the aux, sorted
        for i in (0..src.len()).rev() {
            let ch = Self::char_at(&src[i], pos);
            accumulation[ch] -= 1;
            let new_i = accumulation[ch];

            std::mem::swap(&mut src[i], &mut self.aux[new_i]);
        }

        // Move back to the src
        for i in 0..src.len() {
            std::mem::swap(&mut src[i], &mut self.aux[i]);
        }

        // Recursively sort for the rest of characters
        for i in 1..self.radix {
            Self::sort_(&mut self, src, pos + 1);
        }
    }
}
