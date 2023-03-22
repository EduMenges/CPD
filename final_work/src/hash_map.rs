use crate::hash::MyHash;
use std::collections::LinkedList;

pub struct HashMap<K, V> {
    table: Vec<LinkedList<(K, V)>>,
}

impl<K, V> HashMap<K, V>
where
    K: MyHash + std::cmp::PartialEq,
{
    pub fn new(entries: usize) -> Self {
        let mut table = Vec::new();

        for _ in 0..entries {
            table.push(LinkedList::new())
        }

        Self { table }
    }

    pub fn insert(&mut self, entry: (K, V)) {
        let hashed = entry.0.hash();

        if !self.contains(&entry.0) {
            self.table[hashed].push_back(entry);
        } else {
            self.update_entry(hashed, entry);
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&mut V> {
        let hashed = key.hash();
        self.table[hashed]
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    #[inline]
    pub fn contains(&self, key: &K) -> bool {
        let hashed = key.hash();
        self.table[hashed].iter().any(|(k, _)| k == key)
    }

    #[inline]
    fn update_entry(&mut self, hash: usize, entry: (K, V)) {
        *self.get(&entry.0).unwrap() = entry.1;
    }
}
