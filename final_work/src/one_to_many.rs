use crate::{
    hash::MyHash,
    hash_map::{self, HashMap},
};

#[derive(Debug)]
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
        match self.get_mut(&entry.0) {
            Some(collection) => collection.push(entry.1),
            None => self.map.insert((entry.0, vec![entry.1])),
        }
    }

    pub fn get(&self, key: &K) -> Option<&Vec<V>> {
        self.map.get(key)
    }

    #[inline]
    fn get_mut(&mut self, key: &K) -> Option<&mut Vec<V>> {
        self.map.get_mut(key)
    }

    pub fn update(&mut self, key: &K, value: V) {
        if let Some(vec) = self.get_mut(key) {
            vec.push(value)
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, K, Vec<V>> {
        Iter {
            map_i: self.map.iter(),
        }
    }
}

pub struct Iter<'a, K, V> {
    map_i: hash_map::Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a (K, V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.map_i.next()
    }
}

#[cfg(test)]
mod tests {
    use super::OneToMany;

    #[test]
    fn all() {
        let common_key = String::from("Números");

        let mut otm = OneToMany::new(10);
        otm.insert((common_key.clone(), 1));
        otm.update(&common_key, 2);
        otm.update(&common_key, 3);

        let expected = vec![1, 2, 3];
        assert_eq!(otm.get(&common_key).unwrap(), &expected);
    }
}
