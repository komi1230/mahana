use std::collections::HashMap;
use std::hash::Hash;

pub struct Trie<K: Eq + Hash, V: Eq> {
    value: Option<V>,
    children: HashMap<K, Trie<K, V>>,
}

impl<K, V> Trie<K, V>
where
    K: Eq + Hash,
    V: Eq,
{
    pub fn new() -> Trie<K, V> {
        Trie {
            value: None,
            children: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hoge() {
        assert_eq!(1 + 1, 2);
    }
}
