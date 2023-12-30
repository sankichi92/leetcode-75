pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let (longer, shorter) = if text1.len() >= text2.len() {
        (text1, text2)
    } else {
        (text2, text1)
    };

    let mut lengths = vec![0; shorter.len() + 1];

    for current in longer.chars() {
        for (i, char) in shorter.chars().enumerate() {
            if current == char {
                lengths[i + 1] = lengths[i + 1].max(lengths[i] + 1);
                break;
            }
        }
    }

    *lengths.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        )
    }

    #[test]
    fn case3() {
        assert_eq!(
            longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        )
    }
}
