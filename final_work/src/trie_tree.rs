use array_init::array_init;

/// This amount is enough to represent all [a-z][A-Z] letters.
const AMOUNT_OF_LETTERS: usize = 57;

struct TrieNode<'a, T> {
    children: [Option<Box<TrieNode<'a, T>>>; AMOUNT_OF_LETTERS],
    member: Option<&'a T>,
}

impl<'a, T> TrieNode<'a, T> {
    fn new(member: Option<&'a T>) -> Self {
        Self {
            children: array_init(|_| None),
            member,
        }
    }
}

pub struct Trie<'a, T> {
    root: TrieNode<'a, T>,
}

impl<'a, T> Trie<'a, T> {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(None),
        }
    }

    pub fn insert(&mut self, name: &str, new_member: &'a T) {
        let mut current = &mut self.root;

        for ch in name.chars() {
            let index = Self::char_to_index(ch);
            current = current.children[index].get_or_insert_with(|| Box::new(TrieNode::new(None)));
        }

        current.member = Some(new_member);
    }

    pub fn search(&self, word: &str) -> Option<&'a T> {
        match self.search_node(word) {
            Some(node) => node.member,
            None => None,
        }
    }

    fn search_node(&self, word: &str) -> Option<&TrieNode<'a, T>> {
        let mut node = &self.root;

        for ch in word.chars() {
            let index = Self::char_to_index(ch);

            match &node.children[index] {
                Some(child) => {
                    node = child;
                }
                _ => return None,
            }
        }

        Some(node)
    }

    pub rec_search(&self, word &str) -> Vec<&'a T> {
        let ret = Vec::new();

        base_node = self.search_node(word);
        match base_node {
            
        }
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current = &self.root;

        for ch in prefix.chars() {
            let index = Self::char_to_index(ch);

            if let Some(child) = &current.children[index] {
                current = child;
            } else {
                return false;
            }
        }

        true
    }

    #[inline]
    fn char_to_index(ch: char) -> usize {
        const STARTING_LETTER: char = 'A';
        ch as usize - STARTING_LETTER as usize
    }
}
