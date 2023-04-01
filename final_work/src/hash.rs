pub trait MyHash {
    fn hash(&self) -> usize;
}

impl MyHash for str {
    // Using the djb2 hash function
    fn hash(&self) -> usize {
        let mut hash: usize = 5381;

        for byte in self.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as usize);
        }

        hash
    }
}

impl MyHash for String {
    #[inline(always)]
    fn hash(&self) -> usize {
        self.as_str().hash()
    }
}

impl MyHash for u32 {
    // Using the FNV-1a hash function
    fn hash(&self) -> usize {
        const FNV_OFFSET: u32 = 0x811c_9dc5;
        const FNV_PRIME: u32 = 0x0100_0193;

        let mut hash: u32 = FNV_OFFSET;

        for byte in self.to_ne_bytes() {
            hash = hash.wrapping_mul(FNV_PRIME);
            hash ^= u32::from(byte);
        }

        hash as usize
    }
}
