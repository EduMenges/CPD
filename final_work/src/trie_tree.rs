/// This amount is enough to represent all [a-z][A-Z] letters.
const AMOUNT_OF_LETTERS: usize = 57;

struct TrieNode {
    children: [Option<Box<TrieNode>>; AMOUNT_OF_LETTERS],
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        const CHILDREN_INIT: Option<Box<TrieNode>> = None;
        Self {
            children: [CHILDREN_INIT; AMOUNT_OF_LETTERS],
            is_end_of_word: false,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut current = &mut self.root;

        for ch in word.chars() {
            let index = Self::char_to_index(ch);
            current = current.children[index].get_or_insert_with(|| Box::new(TrieNode::new()));
        }

        current.is_end_of_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut current = &self.root;

        for ch in word.chars() {
            let index = Self::char_to_index(ch);

            if let Some(child) = &current.children[index] {
                current = child;
            } else {
                return false;
            }
        }

        current.is_end_of_word
    }

    pub fn starts_with(&self, prefix: String) -> bool {
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
