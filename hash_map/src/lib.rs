use std::collections::LinkedList;
use std::hash::{Hash, Hasher};

const DEFAULT_CAPACITY: usize = 16;

struct Entry<K, V> {
    key: K,
    value: V,
}

pub struct HashMap<K, V> {
    entries: Vec<LinkedList<Entry<K, V>>>,
    size: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            entries: vec![LinkedList::new(); DEFAULT_CAPACITY],
            size: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.size >= self.entries.len() * 2 {
            self.resize();
        }

        let index = self.hash_index(&key);
        let list = &mut self.entries[index];

        for entry in list.iter_mut() {
            if entry.key == key {
                return Some(std::mem::replace(&mut entry.value, value));
            }
        }

        list.push_back(Entry { key, value });
        self.size += 1;
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash_index(key);
        let list = &self.entries[index];

        for entry in list {
            if entry.key == *key {
                return Some(&entry.value);
            }
        }

        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let index = self.hash_index(key);
        let list = &mut self.entries[index];

        for i in 0..list.len() {
            if list[i].key == *key {
                self.size -= 1;
                return Some(list.remove(i).value);
            }
        }

        None
    }

    fn hash_index(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.entries.len()
    }

    fn resize(&mut self) {
        let mut new_entries = vec![LinkedList::new(); self.entries.len() * 2];
        std::mem::swap(&mut self.entries, &mut new_entries);
        self.size = 0;

        for list in new_entries {
            for entry in list {
                self.insert(entry.key, entry.value);
            }
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
