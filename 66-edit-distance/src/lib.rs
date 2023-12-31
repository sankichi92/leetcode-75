pub fn min_distance(word1: String, word2: String) -> i32 {
    fn min_distance(mut word1: String, mut word2: String, last_popped: char) -> i32 {
        if word1.is_empty() && word2.is_empty() {
            return 0;
        }
    
        let char1 = word1.pop();
        let char2 = word2.pop();
        match (char1, char2) {
            (Some(char1), Some(char2)) => {
                if char1 == char2 || char1 == last_popped  {
                    min_distance(word1, word2, char2)
                } else {
                    1 + min_distance(word1, word2, char2)
                }
            }
            _ => 1 + min_distance(word1, word2, '0'),
        }
    }

    min_distance(word1, word2, '0')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(
            min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
