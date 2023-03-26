use crate::hash::MyHash;
use core::slice;
use std::collections::{linked_list, LinkedList};

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
        let hashed = self.do_hash(&entry.0);

        if !self.contains(&entry.0) {
            self.table[hashed].push_back(entry);
        } else {
            self.update_entry(hashed, entry);
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&mut V> {
        let hashed = self.do_hash(key);

        self.table[hashed]
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    #[inline]
    fn do_hash(&self, key: &K) -> usize {
        key.hash() % self.table.capacity()
    }

    #[inline]
    pub fn contains(&self, key: &K) -> bool {
        let hashed = self.do_hash(key);
        self.table[hashed].iter().any(|(k, _)| k == key)
    }

    #[inline]
    fn update_entry(&mut self, hash: usize, entry: (K, V)) {
        *self.get(&entry.0).unwrap() = entry.1;
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        let vec_i = self.table.iter();
        let list_i = self.table[0].iter();

        Iter { vec_i, list_i }
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    vec_i: slice::Iter<'a, LinkedList<(K, V)>>,
    list_i: linked_list::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.list_i.next() {
            Some(entry)
        } else {
            loop {
                let next_row = self.vec_i.next()?;

                if !next_row.is_empty() {
                    self.list_i = next_row.iter();
                    break;
                }
            }

            self.list_i.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HashMap;

    #[test]
    fn get() {
        let mut map = HashMap::new(10);
        map.insert((1, "Um"));
        let reference = map.get(&1).unwrap();

        assert_eq!(*reference, "Um");
    }

    #[test]
    fn double_insert() {
        let mut map = HashMap::new(10);
        map.insert((1, "Um"));
        map.insert((1, "Uno"));
        let reference = map.get(&1).unwrap();

        assert_eq!(*reference, "Uno");
    }
}
