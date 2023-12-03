pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let max_len = str1.len().min(str2.len());

    'new_candidate: for i in (1..=max_len).rev() {
        if str1.len() % i != 0 || str2.len() % i != 0 {
            continue;
        }

        let candidate: String = str1.chars().take(i).collect();

        for j in 1..(str1.len() / i) {
            let s: String = str1.chars().skip(i * j).take(i).collect();
            if s != candidate {
                continue 'new_candidate;
            }
        }

        for j in 0..(str2.len() / i) {
            let s: String = str2.chars().skip(i * j).take(i).collect();
            if s != candidate {
                continue 'new_candidate;
            }
        }

        return candidate;
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = gcd_of_strings(String::from("ABCABC"), String::from("ABC"));
        assert_eq!(result, "ABC");
    }

    #[test]
    fn case2() {
        let result = gcd_of_strings(String::from("ABABAB"), String::from("ABAB"));
        assert_eq!(result, "AB");
    }

    #[test]
    fn case3() {
        let result = gcd_of_strings(String::from("LEET"), String::from("CODE"));
        assert_eq!(result, "");
    }
}
