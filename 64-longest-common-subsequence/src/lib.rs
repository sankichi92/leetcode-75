pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let (longer, shorter) = if text1.len() >= text2.len() {
        (text1, text2)
    } else {
        (text2, text1)
    };

    let mut lengths = vec![0; shorter.len()];
    // println!("{:?}", shorter.chars().collect::<Vec<char>>());

    for current in longer.chars() {
        for (i, char) in shorter.chars().enumerate() {
            if current == char {
                lengths[i] = lengths.iter().take(i).max().unwrap_or(&0) + 1;
                break;
            }
        }
        // println!("{}: {:?}", current, lengths)
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

    #[test]
    fn failed_case1() {
        assert_eq!(
            longest_common_subsequence("hofubmnylkra".to_string(), "pqhgxgdofcvmr".to_string()),
            5
        )
    }
}
