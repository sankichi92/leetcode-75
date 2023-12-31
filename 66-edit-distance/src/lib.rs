//     r o s
//   0 1 2 3
// h 1 1 2 3
// o 2 2 1 2
// r 3 2 2 2
// s 4 3 3 2
// e 5 4 4 3

//     r o s
//   0 1 2 3
// h 1 1 2 3
// o 2 2 1 2
// r 3 2 2 2
// o 4 3 2 3
// s 4 3 2 2

pub fn min_distance(word1: String, word2: String) -> i32 {
    let (longer, shorter) = if word1.len() >= word2.len() {
        (word1, word2)
    } else {
        (word2, word1)
    };

    if shorter.is_empty() {
        return longer.len() as i32;
    }

    let mut distances: Vec<_> = (0..=shorter.len()).collect();
    let mut prev_distances = distances.clone();

    for l_char in longer.chars() {
        for (j, s_char) in shorter.chars().enumerate() {
            if j == 0 {
                distances[0] += 1
            }
            if l_char == s_char {
                distances[j + 1] = prev_distances[j];
            } else {
                distances[j + 1] = distances[j + 1].min(distances[j]).min(prev_distances[j]) + 1
            }
        }
        prev_distances.copy_from_slice(&distances);
    }

    *distances.last().unwrap() as i32
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

    #[test]
    fn failed_case1() {
        assert_eq!(min_distance("a".to_string(), "ab".to_string()), 1);
    }

    #[test]
    fn failed_case2() {
        assert_eq!(min_distance("".to_string(), "a".to_string()), 1);
    }
}
