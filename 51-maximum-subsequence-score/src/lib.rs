pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let length = nums1.len();
    let mut current_combination: Vec<usize> = (0..k).collect();
    let mut max_score = 0;

    'main: loop {
        let num1: i64 = current_combination.iter().map(|&i| nums1[i] as i64).sum();
        let num2 = current_combination.iter().map(|&i| nums2[i]).min().unwrap() as i64;
        max_score = max_score.max(num1 * num2);

        for i in (0..k).rev() {
            if current_combination[i] < length - k + i {
                current_combination[i] += 1;
                break;
            }

            if i == 0 {
                break 'main;
            }
        }
    }

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3), 12)
    }

    #[test]
    fn case2() {
        assert_eq!(max_score(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1), 30)
    }
}
