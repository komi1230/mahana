use std::collections::HashSet;
use std::hash::Hash;

pub struct Trie<V: Ord + Eq + Hash> {
    value: Option<V>,
    children: Option<HashSet<V>>,
}

impl<V> Trie<V>
where
    V: Ord + Eq + Hash,
{
    pub fn new() -> Trie<V> {
        Trie {
            value: None,
            children: Some(HashSet::new()),
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
