pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums.clone();
    let mut count = 0;

    for i in 0..nums.len() {
        if let Some(current) = nums.get(i) {
            if let Some((j, _)) = nums
                .iter()
                .skip(i + 1)
                .enumerate()
                .find(|(_, &num)| num == k - current)
            {
                nums.remove(i + 1 + j);
                count += 1;
            }
        } else {
            break;
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
