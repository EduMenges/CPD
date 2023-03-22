use std::{collections::LinkedList, fmt::Display};

use crate::hash::MyHash;

pub struct HashTable<V> {
    table: Vec<LinkedList<V>>,
}

impl<V> HashTable<V>
where
    V: MyHash + PartialEq,
{
    pub fn new(entries: usize) -> Self {
        let mut table = Vec::new();

        for _ in 0..entries {
            table.push(LinkedList::new())
        }

        Self { table }
    }

    pub fn insert(&mut self, entry: V) {
        let hashed = entry.hash();
        if !self.contains(&entry) {
            self.table[hashed].push_back(entry);
        }
    }

    #[inline]
    pub fn contains(&self, entry: &V) -> bool {
        let hashed = entry.hash();
        self.table[hashed].contains(entry)
    }
}
