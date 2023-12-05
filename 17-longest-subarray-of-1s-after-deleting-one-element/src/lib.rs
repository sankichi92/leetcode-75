use std::collections::VecDeque;

pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let first_zero_index = match nums.iter().enumerate().find(|(_, &num)| num == 0) {
        Some((i, _)) => i,
        None => return nums.len() as i32 - 1,
    };
    let mut subarray: VecDeque<_> = nums.iter().take(first_zero_index + 1).collect();
    let mut longest = first_zero_index + 1;

    for num in nums.iter().skip(first_zero_index + 1) {
        if *num == 0 {
            while let Some(1) = subarray.pop_front() {}
        }

        subarray.push_back(num);
        longest = longest.max(subarray.len());
    }

    (longest as i32) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(longest_subarray(vec![1, 1, 0, 1]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(longest_subarray(vec![0, 1, 1, 1, 0, 1, 1, 0, 1]), 5);
    }

    #[test]
    fn case3() {
        assert_eq!(longest_subarray(vec![1, 1, 1]), 2);
    }
}
