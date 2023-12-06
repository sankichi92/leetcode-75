pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut left_sum = 0;
    let mut right_sum = nums.iter().sum();

    for (i, num) in nums.iter().enumerate() {
        right_sum -= num;

        if left_sum == right_sum {
            return i as i32;
        }

        left_sum += num;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn case3() {
        assert_eq!(pivot_index(vec![2, 1, -1]), 0);
    }
}
