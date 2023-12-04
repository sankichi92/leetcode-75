pub fn max_vowels(s: String, k: i32) -> i32 {
    let vowel_letters = ['a', 'e', 'i', 'o', 'u'];

    let mut max: i32 = 0;

    for chars in s.chars().collect::<Vec<_>>().windows(k as usize) {
        let count = chars.iter().filter(|c| vowel_letters.contains(c)).count() as i32;
        if count > max {
            max = count;
            if max == k {
                break;
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(max_vowels("abciiidef".to_string(), 3), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(max_vowels("aeiou".to_string(), 2), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(max_vowels("leetcode".to_string(), 3), 2);
    }
}
