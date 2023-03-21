use std::{collections::LinkedList, fmt::Display};

pub trait MyHash {
    fn hash(&self) -> usize;
}

pub struct HashTable<V> {
    table: Vec<LinkedList<V>>,
}

impl<V> HashTable<V>
where
    V: MyHash + Clone + PartialEq,
{
    pub fn new(entries: usize) -> Self {
        Self {
            table: vec![LinkedList::new(); entries],
        }
    }

    pub fn insert(&mut self, entry: V) {
        let hashed = entry.hash();
        self.table[hashed].push_back(entry);
    }

    pub fn contains(&self, entry: &V) -> bool {
        let hashed = entry.hash();
        self.table[hashed].contains(entry)
    }
}
