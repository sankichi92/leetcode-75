pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    if nums.len() == 2 {
        return nums[0].max(nums[1]);
    }

    let (mut cand1, mut cand2, mut prev) = (nums[0], nums[1], nums[0] + nums[2]);

    for num in nums.iter().skip(3) {
        (cand1, cand2, prev) = (cand2, prev, num + cand1.max(cand2));
    }

    (cand1 + nums.last().unwrap()).max(cand2.max(prev))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn case2() {
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn failed_case1() {
        assert_eq!(rob(vec![0]), 0);
    }
}
