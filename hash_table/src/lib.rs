
use std::collections::LinkedList;

const P: u8 = 123;

pub struct HashTable {
    table: Vec<LinkedList<String>>,
}

impl HashTable {
    pub new(entries: usize) -> Self {
        Self {table: Vec::with_capacity(entries),}
    }

    pub insert(&mut self, string: String) {
        let hashed = self.hash(&string);
        self.table[hashed].push(string);
    }

    pub search(&self, string: &str) {
        let hashed = self.hash(string);
        let list = self.table[hashed];
        
        
    }

    hash(&self, string: &str) -> usize {
        let m = self.table.len();
                let mut hash = 0_size;

        for ch in string.chars() {
            hash = (ch as usize) % m + (P * hash) % m;
        }

        hash
    }
}
