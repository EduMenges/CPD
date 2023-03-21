pub struct HashMap<K, V> {
    table: Vec<LinkedList<(K, V)>>,
}

impl<K, V> HashMap<K, V>
where
    K: MyHash,
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
