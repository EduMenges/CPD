use std::borrow::Borrow;

use crate::{
    hash::MyHash,
    hash_map::{self, HashMap},
};

#[derive(Debug)]
pub struct OneToMany<K: MyHash + std::cmp::PartialEq, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> OneToMany<K, V>
where
    K: MyHash + PartialEq,
{
    pub fn new(entries: usize) -> Self {
        Self {
            map: HashMap::new(entries),
        }
    }

    #[inline]
    pub fn insert(&mut self, entry: (K, V)) {
        match self.get_mut(&entry.0) {
            Some(collection) => collection.push(entry.1),
            None => self.map.unchkd_insert((entry.0, vec![entry.1])),
        }
    }

    #[inline(always)]
    pub fn get<Q: ?Sized>(&self, query: &Q) -> Option<&Vec<V>>
    where
        K: Borrow<Q>,
        Q: PartialEq + MyHash,
    {
        self.map.get(query)
    }

    #[inline(always)]
    fn get_mut<Q: ?Sized>(&mut self, query: &Q) -> Option<&mut Vec<V>>
    where
        K: Borrow<Q>,
        Q: MyHash + PartialEq,
    {
        self.map.get_mut(query)
    }

    #[inline(always)]
    pub fn iter(&self) -> Iter<'_, K, Vec<V>> {
        Iter {
            map_i: self.map.iter(),
        }
    }

    pub fn update<Q: ?Sized>(&mut self, query: &Q, val: V)
    where
        K: Borrow<Q>,
        Q: MyHash + PartialEq,
    {
        if let Some(bin) = self.get_mut(query) {
            bin.push(val)
        }
    }
}
pub struct Iter<'a, K, V> {
    map_i: hash_map::Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a (K, V);

    #[inline(always)]
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
