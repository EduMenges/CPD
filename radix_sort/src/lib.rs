mod the_algos;

fn sort(src: &mut[String], lo: usize, hi: usize, pos: usize) {
    let mut aux: Vec<String> = Vec::with_capacity(hi - lo);
    let mut count = [0_u16; 258];
    let char_at = |word: &str, n: usize| -> Option<char> {word.chars().nth(n)};
    
    // Computes the frequency count
    for i in lo..=hi {
        count[char_at(&src[i], pos + 2).unwrap() as usize] += 1;
    }

    // Transform the counts to indices
    for r in 1..count.len() {
        count[r] += count[r - 1];
    }

    // Distribute
    for i in lo..=hi {
        
    }
}

pub fn radix_sort(src: &mut[String]) {
}
