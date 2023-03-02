use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;

        for ch in word.chars() {
            current_node = current_node.children.entry(ch).or_insert(TrieNode::new());
        }

        current_node.is_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut current_node = &self.root;

        for c in word.chars() {
            if let Some(node) = current_node.children.get(&c) {
                current_node = node;
            } else {
                return false;
            }
        }

        current_node.is_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut trie = Trie::new();
        trie.insert("hello".to_string());
        trie.insert("world".to_string());

        assert!(trie.search("hello"));
        assert!(trie.search("world"));
    }
}
