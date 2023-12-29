use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let digit_to_letters = HashMap::from([
        ('2', vec!['a', 'b', 'c']),
        ('3', vec!['d', 'e', 'f']),
        ('4', vec!['g', 'h', 'i']),
        ('5', vec!['j', 'k', 'l']),
        ('6', vec!['m', 'n', 'o']),
        ('7', vec!['p', 'q', 'r', 's']),
        ('8', vec!['t', 'u', 'v']),
        ('9', vec!['w', 'x', 'y', 'z']),
    ]);

    let init = digit_to_letters
        .get(&digits.chars().next().unwrap())
        .unwrap()
        .iter()
        .map(|c| c.to_string())
        .collect();

    digits
        .chars()
        .skip(1)
        .map(|char| digit_to_letters.get(&char).unwrap().clone())
        .fold(init, |results, letters| {
            let mut new_results = vec![];
            for str in results {
                for letter in letters.iter() {
                    let mut str = str.clone();
                    str.push(*letter);
                    new_results.push(str);
                }
            }
            new_results
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(letter_combinations("".to_string()), Vec::<String>::new());
    }

    #[test]
    fn case3() {
        assert_eq!(letter_combinations("2".to_string()), vec!["a", "b", "c"]);
    }
}
