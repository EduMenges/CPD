use std::iter::Map;

use array_init::array_init;

/// This amount is enough to represent the ASCII characters from space (32) to z (122)
const AMOUNT_OF_LETTERS: usize = 91;

struct TrieNode<T> {
    children: [Option<Box<TrieNode<T>>>; AMOUNT_OF_LETTERS],
    member: Option<*const T>,
}

impl<T> TrieNode<T> {
    pub fn new(member: Option<*const T>) -> Self {
        Self {
            children: array_init(|_| None),
            member,
        }
    }

    pub fn dfs(&self, buffer: &mut Vec<*const T>) {
        for node in self.children.iter().filter_map(|child| child.as_ref()) {
            if let Some(member) = node.member {
                buffer.push(member)
            }
            node.dfs(buffer);
        }
    }
}

pub struct Trie<T> {
    root: TrieNode<T>,
}

impl<T> Trie<T> {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(None),
        }
    }

    pub fn insert(&mut self, word: &str, new_member: *const T) {
        let mut current = &mut self.root;

        for index in Self::word_to_indexes(word) {
            current = current.children[index].get_or_insert_with(|| Box::new(TrieNode::new(None)));
        }

        current.member = Some(new_member);
    }

    pub fn search(&self, word: &str) -> Option<*const T> {
        match self.search_node(word) {
            Some(node) => node.member,
            None => None,
        }
    }

    fn search_node(&self, word: &str) -> Option<&TrieNode<T>> {
        let mut node = &self.root;

        for index in Self::word_to_indexes(word) {
            match &node.children[index] {
                Some(child) => {
                    node = child;
                }
                _ => return None,
            }
        }

        Some(node)
    }

    pub fn search_all(&self, word: &str) -> Vec<*const T> {
        let mut ret = Vec::new();

        let base_node = self.search_node(word);
        if let Some(node) = base_node {
            node.dfs(&mut ret);
        }

        ret
    }

    #[inline(always)]
    fn char_to_index(ch: char) -> usize {
        const STARTING_LETTER: usize = ' ' as usize;
        ch as usize - STARTING_LETTER
    }

    fn word_to_indexes(word: &str) -> impl Iterator<Item = usize> + '_ {
        word.chars().map(|ch| Self::char_to_index(ch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        let mut trie = Trie::new();
        let bools = [true; 5];

        trie.insert("abc", &bools[0]);
        trie.insert("abcde", &bools[1]);
        trie.insert("abde", &bools[2]);
        trie.insert("b", &bools[3]);
        trie.insert("a", &bools[4]);

        unsafe {
            assert!(*trie.search("abc").unwrap());
            assert!(*trie.search("b").unwrap());
        }
        assert!(trie.search("xyz").is_none());
    }
}
