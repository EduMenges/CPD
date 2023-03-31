use crate::hash::MyHash;
use core::slice;
use std::{
    collections::{linked_list, LinkedList},
    fmt::Display,
};

pub struct ListStats {
    used_entries: usize,
    empty_entries: usize,
    occupy_rate: f64,
    min: usize,
    max: usize,
    avg: f64,
}

impl Display for ListStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NUMERO DE ENTRADAS DA TABELA USADAS {}
NÚMERO DE ENTRADAS DA TABELA VAZIAS {}
TAXA DE OCUPACAO {:.2}
MINIMO TAMANHO DE LISTA {}
MAXIMO TAMANHO DE LISTA {}
MEDIO TAMANHO DE LISTA {:.2}",
            self.used_entries, self.empty_entries, self.occupy_rate, self.min, self.max, self.avg
        )
    }
}

// TODO: mudar implementação para usar vetor de vetor
#[derive(Debug)]
pub struct HashMap<K: MyHash + std::cmp::PartialEq, V> {
    table: Vec<LinkedList<(K, V)>>,
}

impl<K, V> HashMap<K, V>
where
    K: MyHash + std::cmp::PartialEq,
{
    pub fn new(capacity: usize) -> Self {
        let mut table = Vec::with_capacity(capacity);

        for _ in 0..capacity {
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

    pub fn get(&self, key: &K) -> Option<&V> {
        let hashed = self.do_hash(key);

        self.table[hashed]
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let hashed = self.do_hash(key);

        self.table[hashed]
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    #[inline(always)]
    fn do_hash(&self, key: &K) -> usize {
        key.hash() % self.table.capacity()
    }

    #[inline]
    pub fn contains(&self, key: &K) -> bool {
        let hashed = self.do_hash(key);
        self.table[hashed].iter().any(|(k, _)| k == key)
    }

    #[inline(always)]
    fn update_entry(&mut self, hash: usize, entry: (K, V)) {
        *self.get_mut(&entry.0).unwrap() = entry.1;
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        let mut vec_i = self.table.iter();
        let list_i = vec_i.next().unwrap().iter();

        Iter { vec_i, list_i }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        let mut vec_i = self.table.iter_mut();
        let list_i = vec_i.next().unwrap().iter_mut();

        IterMut { vec_i, list_i }
    }
    #[inline]
    pub fn get_empties(&self) -> usize {
        self.table.iter().filter(|l| l.is_empty()).count()
    }

    #[inline]
    pub fn get_used(&self) -> usize {
        self.table.iter().filter(|l| !l.is_empty()).count()
    }

    pub fn get_list_statistics(&self) -> ListStats {
        let used_entries = self.get_used();
        let empty_entries = self.get_empties();
        let occupy_rate = used_entries as f64 / self.table.capacity() as f64;

        let mut min = self.table[0].len();
        let mut max = self.table[0].len();
        let mut total_entries = 0;
        let mut avg = 0.0;
        let total_lists = used_entries as f64;

        self.table.iter().filter(|l| !l.is_empty()).for_each(|l| {
            total_lists += l.len();
            if l.len() < min {
                min = l.len();
            }
            if l.len() > max {
                max = l.len();
            }
            avg += l.len() as f64 / total_lists
        });

        ListStats {
            min,
            max,
            avg,
            used_entries,
            empty_entries,
            occupy_rate,
        }
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

pub struct IterMut<'a, K: 'a, V: 'a> {
    vec_i: slice::IterMut<'a, LinkedList<(K, V)>>,
    list_i: linked_list::IterMut<'a, (K, V)>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = &'a mut (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.list_i.next() {
            Some(entry)
        } else {
            loop {
                let next_row = self.vec_i.next()?;

                if !next_row.is_empty() {
                    self.list_i = next_row.iter_mut();
                    break;
                }
            }

            self.list_i.next()
        }
    }
}

impl<K: MyHash + std::cmp::PartialEq, V> Drop for HashMap<K, V> {
    fn drop(&mut self) {
        let stats = HashMap::<K, V>::get_list_statistics(&self);
        println!("{}", stats);
    }
}

#[cfg(test)]
mod tests {
    use super::HashMap;

    #[test]
    fn get() {
        let mut map = HashMap::new(10);
        map.insert((1, "Um"));
        let reference = map.get_mut(&1).unwrap();

        assert_eq!(*reference, "Um");
    }

    #[test]
    fn double_insert() {
        let mut map = HashMap::new(10);
        map.insert((1, "Um"));
        map.insert((1, "Uno"));
        let reference = map.get_mut(&1).unwrap();

        assert_eq!(*reference, "Uno");
    }
}
