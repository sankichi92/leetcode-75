fn main() {}

pub fn reverse_vowels(s: String) -> String {
    let mut vowels = Vec::new();
    for c in s.chars() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
            vowels.push(c);
        }
    }

    let mut results = String::with_capacity(s.len());
    for c in s.chars() {
        results.push(if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
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
}
