pub trait MyHash {
    fn hash(&self) -> usize;
}

impl MyHash for str {
    fn hash(&self) -> usize {
        let mut hash: usize = 5381;

        for c in self.chars() {
            hash = (hash << 5).wrapping_add(hash) ^ (c as usize);
        }

        hash
    }
}

impl MyHash for String {
    #[inline]
    fn hash(&self) -> usize {
        self.as_str().hash()
    }
}

impl MyHash for u32 {
    fn hash(&self) -> usize {
        const FNV_PRIME: u32 = 0x01000193;
        const FNV_OFFSET: u32 = 0x811c_9dc5;

        let mut hash: u32 = FNV_OFFSET;

        hash ^= self & 0xff;
        hash = hash.wrapping_mul(FNV_PRIME);

        hash ^= (self >> 8) & 0xff;
        hash = hash.wrapping_mul(FNV_PRIME);

        hash ^= (self >> 16) & 0xff;
        hash = hash.wrapping_mul(FNV_PRIME);

        hash ^= self >> 24;
        hash = hash.wrapping_mul(FNV_PRIME);

        hash as usize
    }
}
