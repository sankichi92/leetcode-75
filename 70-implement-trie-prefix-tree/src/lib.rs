use std::collections::HashSet;

pub struct Trie {
    words: HashSet<String>,
}

impl Trie {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Trie {
            words: HashSet::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        self.words.insert(word);
    }

    pub fn search(&self, word: String) -> bool {
        self.words.contains(&word)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.words.iter().any(|word| word.starts_with(&prefix))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }
}
