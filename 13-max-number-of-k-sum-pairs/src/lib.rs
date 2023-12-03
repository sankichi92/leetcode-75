pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0;

    let mut nums = nums.clone();
    nums.sort();

    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = nums[left] + nums[right];
        match sum {
            _ if sum < k => left += 1,
            _ if sum > k => right -= 1,
            _ => {
                count += 1;
                left += 1;
                right -= 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(max_operations(vec![1, 2, 3, 4], 5), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    }
}
