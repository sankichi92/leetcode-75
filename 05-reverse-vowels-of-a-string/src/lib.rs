pub fn reverse_vowels(s: String) -> String {
    let mut vowels = Vec::new();
    for c in s.chars() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&c.to_ascii_lowercase()) {
            vowels.push(c);
        }
    }

    let mut results = String::with_capacity(s.len());
    for c in s.chars() {
        results.push(if ['a', 'e', 'i', 'o', 'u'].contains(&c.to_ascii_lowercase()) {
            vowels.pop().unwrap()
        } else {
            c
        });
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result: String = reverse_vowels("hello".to_string());
        assert_eq!(result, "holle");
    }

    #[test]
    fn case2() {
        let result: String = reverse_vowels("leetcode".to_string());
        assert_eq!(result, "leotcede");
    }

    #[test]
    fn failed_case1() {
        let result: String = reverse_vowels("aA".to_string());
        assert_eq!(result, "Aa");
    }
}
