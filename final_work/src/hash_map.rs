use crate::hash::MyHash;
use core::slice;
use std::borrow::Borrow;
use std::intrinsics::likely;

#[derive(Debug)]
pub struct ListStats {
    used_rows: usize,
    empty_rows: usize,
    occupy_rate: f64,
    total_entries: usize,
    min: usize,
    max: usize,
    avg: f64,
}

#[derive(Debug)]
pub struct HashMap<K: MyHash + PartialEq, V> {
    table: Vec<Vec<(K, V)>>,
}

impl<K, V> HashMap<K, V>
where
    K: MyHash + PartialEq,
{
    pub fn new(capacity: usize) -> Self {
        let mut table = Vec::with_capacity(capacity);

        for _ in 0..capacity {
            table.push(Vec::new());
        }

        Self { table }
    }

    #[inline(always)]
    pub fn unchkd_insert(&mut self, entry: (K, V)) {
        let hashed = self.do_hash(&entry.0);
        self.table[hashed].push(entry);
    }

    pub fn insert(&mut self, entry: (K, V)) {
        if self.contains(&entry.0) {
            self.update_entry(entry);
        } else {
            self.unchkd_insert(entry);
        }
    }

    #[inline(always)]
    pub fn get<Q: ?Sized>(&self, query: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: MyHash + PartialEq,
    {
        self.find(query).map(|(i, j)| &self.table[i][j].1)
    }

    #[inline(always)]
    pub fn get_mut<Q: ?Sized>(&mut self, query: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: MyHash + PartialEq,
    {
        self.find(query).map(|(i, j)| &mut self.table[i][j].1)
    }

    fn find<Q: ?Sized>(&self, query: &Q) -> Option<(usize, usize)>
    where
        K: Borrow<Q>,
        Q: MyHash + PartialEq,
    {
        let hashed = self.do_hash(query);

        for (i, (key, _)) in self.table[hashed].iter().enumerate() {
            if likely(query == key.borrow()) {
                return Some((hashed, i));
            }
        }
        None
    }

    #[inline(always)]
    fn do_hash<Q: ?Sized>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: MyHash + PartialEq,
    {
        key.hash() % self.table.capacity()
    }

    #[inline]
    pub fn contains<Q: ?Sized>(&self, query: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: MyHash + PartialEq,
    {
        self.find(query).is_some()
    }

    #[inline(always)]
    fn update_entry(&mut self, entry: (K, V)) {
        *self.get_mut(&entry.0).unwrap() = entry.1;
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        let mut vec_i = self.table.iter();
        let list_i = vec_i.next().unwrap().iter();

        Iter {
            bin_i: vec_i,
            row_i: list_i,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        let mut vec_i = self.table.iter_mut();
        let list_i = vec_i.next().unwrap().iter_mut();

        IterMut {
            bin_i: vec_i,
            row_i: list_i,
        }
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
        let used_rows = self.get_used();
        let empty_rows = self.get_empties();
        let occupy_rate = used_rows as f64 / self.table.capacity() as f64;

        let total_lists = used_rows as f64;
        let mut min = self.table[0].len();
        let mut max = self.table[0].len();
        let mut total_entries = 0;
        let mut avg = 0.0;

        self.table
            .iter()
            .filter(|row| !row.is_empty())
            .for_each(|l| {
                total_entries += l.len();

                if l.len() < min {
                    min = l.len();
                } else if l.len() > max {
                    max = l.len();
                }

                avg += l.len() as f64 / total_lists;
            });

        ListStats {
            min,
            max,
            avg,
            used_rows,
            empty_rows,
            occupy_rate,
            total_entries,
        }
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    bin_i: slice::Iter<'a, Vec<(K, V)>>,
    row_i: slice::Iter<'a, (K, V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.row_i.next() {
            Some(entry)
        } else {
            loop {
                let next_row = self.bin_i.next()?;

                if !next_row.is_empty() {
                    self.row_i = next_row.iter();
                    break;
                }
            }

            self.row_i.next()
        }
    }
}

pub struct IterMut<'a, K: 'a, V: 'a> {
    bin_i: slice::IterMut<'a, Vec<(K, V)>>,
    row_i: slice::IterMut<'a, (K, V)>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = &'a mut (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entry) = self.row_i.next() {
            Some(entry)
        } else {
            loop {
                let next_row = self.bin_i.next()?;

                if !next_row.is_empty() {
                    self.row_i = next_row.iter_mut();
                    break;
                }
            }

            self.row_i.next()
        }
    }
}

impl<K: MyHash + std::cmp::PartialEq, V> Drop for HashMap<K, V> {
    fn drop(&mut self) {
        let stats = HashMap::<K, V>::get_list_statistics(&self);
        println!("{:?}", stats);
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
