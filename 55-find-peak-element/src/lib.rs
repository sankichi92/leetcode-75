pub fn find_peak_element(mut nums: Vec<i32>) -> i32 {
    let length = nums.len();
    nums.insert(0, i32::MIN);
    nums.push(i32::MIN);
    nums.iter()
        .skip(1)
        .take(length)
        .enumerate()
        .find(|(i, num)| nums[*i] <= **num && **num >= nums[*i + 2])
        .unwrap()
        .0 as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(find_peak_element(vec![1, 2, 3, 1]), 2)
    }

    #[test]
    fn case2() {
        assert_eq!(find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 1)
    }

    #[test]
    fn failed_case1() {
        assert_eq!(find_peak_element(vec![-2147483648]), 0)
    }
}
