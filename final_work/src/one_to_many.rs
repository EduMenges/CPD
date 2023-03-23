use crate::{hash::MyHash, hash_map::HashMap};

pub struct OneToMany<K, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> OneToMany<K, V>
where
    K: MyHash + std::cmp::PartialEq,
{
    pub fn new(entries: usize) -> Self {
        Self {
            map: HashMap::new(entries),
        }
    }

    pub fn insert(&mut self, entry: (K, V)) {
        match self.map.get(&entry.0) {
            Some(collection) => collection.push(entry.1),
            None => self.map.insert((entry.0, vec![entry.1])),
        }
    }
}
