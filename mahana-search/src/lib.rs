use std::collections::HashMap;

pub struct Trie {
    children: HashMap<char, TrieNode>,
}

pub struct TrieNode {
    value: Option<char>,
    children: HashMap<char, TrieNode>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hoge() {
        assert_eq!(1 + 1, 2);
    }
}
