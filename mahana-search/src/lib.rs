use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
pub struct Trie {
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, data: &str) {
        let mut cs = data.chars();
        if let Some(mut node) = self.children.get(&cs.next().unwrap()) {
            while let Some(c) = cs.next() {
                if let Some(v) = node.children.get(&c) {
                    node = v;
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Trie::new(),
            Trie {
                children: HashMap::new()
            }
        );
    }
}
