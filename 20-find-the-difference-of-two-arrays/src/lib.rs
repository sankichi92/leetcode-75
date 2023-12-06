use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut set1 = HashSet::new();
    for num in nums1 {
        set1.insert(num);
    }

    let mut set2 = HashSet::new();
    for num in nums2 {
        set2.insert(num);
    }

    vec![
        set1.difference(&set2).copied().collect(),
        set2.difference(&set1).copied().collect(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut results = find_difference(vec![1, 2, 3], vec![2, 4, 6]);
        results[0].sort();
        results[1].sort();
        assert_eq!(results[0], vec![1, 3]);
        assert_eq!(results[1], vec![4, 6]);
    }

    #[test]
    fn case2() {
        let mut results = find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]);
        results[0].sort();
        results[1].sort();
        assert_eq!(results[0], vec![3]);
        assert_eq!(results[1], vec![]);
    }
}
