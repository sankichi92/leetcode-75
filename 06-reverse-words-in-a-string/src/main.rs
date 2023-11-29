fn main() {}

pub fn reverse_words(s: String) -> String {
    let reversed: Vec<_> = s.split_ascii_whitespace().rev().collect();
    reversed.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            reverse_words("the sky is blue".to_string()),
            "blue is sky the"
        );
    }

    #[test]
    fn case2() {
        assert_eq!(reverse_words("  hello world  ".to_string()), "world hello");
    }

    #[test]
    fn case3() {
        assert_eq!(
            reverse_words("a good   example".to_string()),
            "example good a"
        );
    }
}
