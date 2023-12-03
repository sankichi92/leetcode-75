pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let max = nums.windows(k as usize).fold(i32::MIN, |max, nums| {
        let sum = nums.iter().sum();
        if sum > max {
            sum
        } else {
            max
        }
    });

    max as f64 / k as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.75);
    }

    #[test]
    fn case2() {
        assert_eq!(find_max_average(vec![5], 1), 5.0);
    }

    #[test]
    fn failed_case1() {
        assert_eq!(find_max_average(vec![-1], 1), -1.0)
    }
}
