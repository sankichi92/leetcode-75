pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut max_len = 0;
    let mut len = 0;
    let mut zero_count = 0;
    let mut first_zero_index = 0;

    'root: loop {
        for (i, &num) in nums.iter().enumerate().skip(first_zero_index + 1) {
            if num == 0 {
                zero_count += 1;
                if zero_count == 1 {
                    first_zero_index = i;
                }
                if zero_count > k {
                    zero_count = 0;
                    len = 0;
                    break;
                }
            }

            len += 1;
            if len > max_len {
                max_len = len;
            }

            if i == nums.len() - 1 {
                break 'root;
            }
        }
    }

    max_len
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
}
