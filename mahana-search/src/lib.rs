use std::collections::HashMap;
use std::hash::Hash;

pub struct Trie<K: Eq + Hash, V: Eq> {
    value: Option<V>,
    children: HashMap<K, Trie<K, V>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hoge() {
        assert_eq!(1 + 1, 2);
    }
}
