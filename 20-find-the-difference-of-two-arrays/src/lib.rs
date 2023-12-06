use std::collections::HashSet;

pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let nums1: HashSet<i32> = HashSet::from_iter(nums1);
    let nums2: HashSet<i32> = HashSet::from_iter(nums2);

    vec![
        nums1.difference(&nums2).copied().collect(),
        nums2.difference(&nums1).copied().collect(),
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
