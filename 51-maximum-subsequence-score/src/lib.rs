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
                for j in (i + 1)..k {
                    current_combination[j] = current_combination[i] + j - i
                }
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

    #[test]
    fn failed_case1() {
        assert_eq!(
            max_score(vec![22, 5, 25, 15, 28, 1], vec![22, 30, 25, 25, 9, 18], 3),
            1364
        )
    }

    #[test]
    fn failed_case2() {
        assert_eq!(
            max_score(
                vec![
                    93, 463, 179, 2488, 619, 2006, 1561, 137, 53, 1765, 2304, 1459, 1768, 450,
                    1938, 2054, 466, 331, 670, 1830, 1550, 1534, 2164, 1280, 2277, 2312, 1509, 867,
                    2223, 1482, 2379, 1032, 359, 1746, 966, 232, 67, 1203, 2474, 944, 1740, 1775,
                    1799, 1156, 1982, 1416, 511, 1167, 1334, 2344
                ],
                vec![
                    345, 229, 976, 2086, 567, 726, 1640, 2451, 1829, 77, 1631, 306, 2032, 2497,
                    551, 2005, 2009, 1855, 1685, 729, 2498, 2204, 588, 474, 693, 30, 2051, 1126,
                    1293, 1378, 1693, 1995, 2188, 1284, 1414, 1618, 2005, 1005, 1890, 30, 895, 155,
                    526, 682, 2454, 278, 999, 1417, 1682, 995
                ],
                42
            ),
            0
        )
    }
}
