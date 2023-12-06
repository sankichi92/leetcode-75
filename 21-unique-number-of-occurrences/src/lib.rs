use std::collections::{HashMap, HashSet};

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for n in arr {
        match map.get_mut(&n) {
            Some(occurrence) => {
                *occurrence += 1;
            }
            None => {
                map.insert(n, 0);
            }
        }
    }

    let occurrences = map.values();
    occurrences.len() == occurrences.into_iter().collect::<HashSet<_>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
    }

    #[test]
    fn case2() {
        assert!(!unique_occurrences(vec![1, 2]));
    }

    #[test]
    fn case3() {
        assert!(unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]));
    }
}
