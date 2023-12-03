pub fn is_subsequence(s: String, t: String) -> bool {
    let mut s: Vec<_> = s.chars().rev().collect();

    let mut current;
    if let Some(first) = s.pop() {
        current = first
    } else {
        return true;
    };

    for char in t.chars() {
        if char == current {
            if let Some(next) = s.pop() {
                current = next;
            } else {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(is_subsequence("abc".to_string(), "ahbgdc".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!is_subsequence("axc".to_string(), "ahbgdc".to_string()));
    }
}
