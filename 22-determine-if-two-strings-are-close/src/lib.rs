use std::collections::HashSet;

pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let word1: HashSet<_> = word1.chars().collect();
    let word2: HashSet<_> = word2.chars().collect();

    if word1 != word2 {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(close_strings("abc".to_string(), "bca".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!close_strings("a".to_string(), "aa".to_string()));
    }

    #[test]
    fn case3() {
        assert!(close_strings("cabbba".to_string(), "abbccc".to_string()));
    }
}
