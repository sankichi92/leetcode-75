use std::collections::VecDeque;

pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut max = 0;
    let mut current = VecDeque::new();
    let mut zero_count = 0;

    for num in nums {
        current.push_back(num);

        if num == 0 {
            if zero_count < k {
                zero_count += 1;
            } else {
                while let Some(1) = current.pop_front() {}
            }
        }

        if current.len() > max {
            max = current.len();
        }
    }

    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
    }

    #[test]
    fn case2() {
        assert_eq!(
            longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            ),
            10
        );
    }

    #[test]
    fn failed_case1() {
        assert_eq!(longest_ones(vec![0, 0, 1, 1, 1, 0, 0], 0), 3);
    }
}
