use std::{collections::LinkedList, fmt::Display};

pub trait MyHash<T> {
    fn hash(var: &T) -> usize;
}

pub struct HashTable<V> {
    table: Vec<LinkedList<V>>,
}

// pub struct ListStats {
//     used_entries: usize,
//     empty_entries: usize,
//     occupy_rate: f64,
//     min: usize,
//     max: usize,
//     avg: f64,
// }

// impl Display for ListStats {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "NUMERO DE ENTRADAS DA TABELA USADAS {}
// NÚMERO DE ENTRADAS DA TABELA VAZIAS {}
// TAXA DE OCUPACAO {:.2}
// MINIMO TAMANHO DE LISTA {}
// MAXIMO TAMANHO DE LISTA {}
// MEDIO TAMANHO DE LISTA {:.2}",
//             self.used_entries, self.empty_entries, self.occupy_rate, self.min, self.max, self.avg
//         )
//     }
// }

impl HashTable<K, V> {
    pub fn new(entries: usize) -> Self {
        Self {
            table: vec![LinkedList::new(); entries],
        }
    }

    pub fn insert(&mut self, string: String) {
        let hashed = self.hash(&string);
        self.table[hashed].push_back(string);
    }

    pub fn search(&self, string: &str) -> i16 {
        let hashed = self.hash(string);
        let list = &self.table[hashed];
        let mut res = 1;
        let mut it = list.iter();
        let mut found = false;

        while !found && let Some(element) = it.next() {
            res += 1;
            if *element == string {
                found = true;
            }
        }

        if !found {
            res = -1;
        }

        res
    }

    // #[inline]
    // pub fn get_empties(&self) -> usize {
    //     self.table.iter().filter(|l| l.is_empty()).count()
    // }

    // #[inline]
    // pub fn get_used(&self) -> usize {
    //     self.table.iter().filter(|l| !l.is_empty()).count()
    // }

    // pub fn get_list_statistics(&self) -> ListStats {
    //     let used_entries = self.get_used();
    //     let empty_entries = self.get_empties();
    //     let occupy_rate = used_entries as f64 / self.table.capacity() as f64;

    //     let mut min = self.table[0].len();
    //     let mut max = self.table[0].len();
    //     let mut avg = 0.0;
    //     let total_lists = used_entries as f64;

    //     self.table.iter().filter(|l| !l.is_empty()).for_each(|l| {
    //         if l.len() < min {
    //             min = l.len();
    //         }
    //         if l.len() > max {
    //             max = l.len();
    //         }
    //         avg += l.len() as f64 / total_lists
    //     });

    //     ListStats {
    //         min,
    //         max,
    //         avg,
    //         used_entries,
    //         empty_entries,
    //         occupy_rate,
    //     }
    // }

    pub fn test_slice(&self, slice: &[String]) {
        for string in slice {
            self.test_search(&string);
        }
    }

    fn test_search(&self, string: &str) {
        let result = self.search(string);

        let display = if result < 0 {
            format!("MISS")
        } else {
            format!("HIT {}", result)
        };

        println!("{string} {display}");
    }

    fn hash(&self, string: &str) -> usize {
        let m = self.table.len();
        let mut hash = 0_usize;

        for ch in string.bytes() {
            hash = ((ch as usize) % m + (P * hash) % m) % m;
        }

        hash
    }
}
