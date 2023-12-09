use std::collections::{HashMap, HashSet};

pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }

    let char_count1 = word1.chars().fold(HashMap::new(), |mut map, char| {
        let entry = map.entry(char).or_insert(0);
        *entry += 1;
        map
    });

    let char_count2 = word2.chars().fold(HashMap::new(), |mut map, char| {
        let entry = map.entry(char).or_insert(0);
        *entry += 1;
        map
    });

    if char_count1.keys().collect::<HashSet<_>>() != char_count2.keys().collect::<HashSet<_>>() {
        return false;
    }

    if char_count1.values().collect::<HashSet<_>>() != char_count2.values().collect::<HashSet<_>>()
    {
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

    #[test]
    fn failed_case1() {
        assert!(!close_strings("abbzzca".to_string(), "babzzcz".to_string()));
    }
}
