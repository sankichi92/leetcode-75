pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    nums[nums.len() - k as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5)
    }

    #[test]
    fn case2() {
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4)
    }
}
