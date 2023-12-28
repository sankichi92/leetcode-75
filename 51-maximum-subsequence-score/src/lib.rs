use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut sorted_indices: Vec<_> = (0..nums1.len()).collect();
    sorted_indices.sort_unstable_by(|&a, &b| match nums2[b].cmp(&nums2[a]) {
        Ordering::Equal => nums1[b].cmp(&nums1[a]),
        res => res,
    });

    let mut nums1_heap: BinaryHeap<_> = sorted_indices
        .iter()
        .take(k)
        .map(|&i| Reverse(nums1[i]))
        .collect();
    let mut nums1_sum: i32 = nums1_heap.iter().map(|num| num.0).sum();
    let mut max_score = nums1_sum as i64 * nums2[sorted_indices[k - 1]] as i64;

    for &i in sorted_indices.iter().skip(k) {
        let (num1, num2) = (nums1[i], nums2[i]);
        let mut min_num1_rev = nums1_heap.peek_mut().unwrap();
        let new_nums1_sum = nums1_sum - min_num1_rev.0 + num1;
        let new_score = new_nums1_sum as i64 * num2 as i64;

        if new_score > max_score {
            min_num1_rev.0 = num1;
            nums1_sum = new_nums1_sum;
            max_score = new_score;
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
                    1799, 1156, 1982, 1416, 511, 1167, 1334, 2344,
                ],
                vec![
                    345, 229, 976, 2086, 567, 726, 1640, 2451, 1829, 77, 1631, 306, 2032, 2497,
                    551, 2005, 2009, 1855, 1685, 729, 2498, 2204, 588, 474, 693, 30, 2051, 1126,
                    1293, 1378, 1693, 1995, 2188, 1284, 1414, 1618, 2005, 1005, 1890, 30, 895, 155,
                    526, 682, 2454, 278, 999, 1417, 1682, 995,
                ],
                42,
            ),
            26653494
        )
    }

    #[test]
    fn failed_case3() {
        assert_eq!(max_score(vec![2, 1, 14, 12], vec![11, 7, 13, 6], 3), 168)
    }

    #[test]
    fn failed_case4() {
        assert_eq!(
            max_score(
                vec![79, 76, 41, 28, 41, 66, 44, 30, 25],
                vec![25, 0, 69, 67, 55, 0, 9, 77, 26],
                7
            ),
            2592
        )
    }
}
