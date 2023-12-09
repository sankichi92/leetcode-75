pub fn decode_string(s: String) -> String {
    let mut results = String::new();
    let mut k_str = String::new();
    let mut encoded_string = String::new();
    let mut bracket_count = 0;

    for char in s.chars() {
        if bracket_count == 0 {
            match char {
                '[' => bracket_count += 1,
                char if char.is_ascii_digit() => k_str.push(char),
                _ => results.push(char),
            }
        } else {
            match char {
                '[' => {
                    bracket_count += 1;
                    encoded_string.push(char);
                }
                ']' => {
                    bracket_count -= 1;
                    if bracket_count == 0 {
                        let k = k_str.parse().unwrap();
                        let s = decode_string(encoded_string.clone());

                        for _ in 0..k {
                            results += &s
                        }

                        k_str.clear();
                        encoded_string.clear();
                    } else {
                        encoded_string.push(char);
                    }
                }
                _ => encoded_string.push(char),
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(decode_string("3[a]2[bc]".to_string()), "aaabcbc");
    }

    #[test]
    fn case2() {
        assert_eq!(decode_string("3[a2[c]]".to_string()), "accaccacc");
    }

    #[test]
    fn case3() {
        assert_eq!(decode_string("2[abc]3[cd]ef".to_string()), "abcabccdcdcdef");
    }
}
