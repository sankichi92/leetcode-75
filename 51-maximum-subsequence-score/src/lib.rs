use std::{
    cmp::{Ord, Ordering},
    collections::{BinaryHeap, HashSet},
};

pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut max_score_indices: HashSet<_> = (0..k).collect();
    let mut nums1_sum: i32 = nums1.iter().take(k).sum();
    let mut heap1: BinaryHeap<_> = nums1
        .iter()
        .enumerate()
        .take(k)
        .map(|(i, &val)| ValAndIdx(val, i))
        .collect();
    let mut heap2: BinaryHeap<_> = nums2
        .iter()
        .enumerate()
        .take(k)
        .map(|(i, &val)| ValAndIdx(val, i))
        .collect();
    let mut max_score = nums1_sum as i64 * heap2.peek().unwrap().0 as i64;

    for i in k..nums1.len() {
        let (num1, num2) = (nums1[i], nums2[i]);

        let val_and_idx1 = heap1.peek().unwrap();
        let val_and_idx2 = heap2.pop().unwrap();

        let score1 = (nums1_sum - val_and_idx1.0 + num1) as i64 * num2.min(val_and_idx2.0) as i64;
        let score2 = (nums1_sum - nums1[val_and_idx2.1] + num1) as i64
            * num2.min(heap2.peek().unwrap_or(&ValAndIdx(num2, i)).0) as i64;

        if score1 > max_score && score1 > score2 {
            max_score = score1;
            nums1_sum = nums1_sum - val_and_idx1.0 + num1;
            max_score_indices.remove(&val_and_idx1.1);
            max_score_indices.insert(i);
            heap2 = max_score_indices
                .iter()
                .map(|&i| ValAndIdx(nums2[i], i))
                .collect();
            heap1.pop();
            heap1.push(ValAndIdx(num1, i));
        } else if score2 > max_score && score2 > score1 {
            max_score = score2;
            nums1_sum = nums1_sum - nums1[val_and_idx2.1] + num1;
            max_score_indices.remove(&val_and_idx2.1);
            max_score_indices.insert(i);
            heap1 = max_score_indices
                .iter()
                .map(|&i| ValAndIdx(nums2[i], i))
                .collect();
            heap2.push(ValAndIdx(num2, i));
        } else {
            heap2.push(val_and_idx2);
        }
    }

    max_score
}

struct ValAndIdx(i32, usize);

impl Ord for ValAndIdx {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for ValAndIdx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ValAndIdx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ValAndIdx {}

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
            1950210
        )
    }
}
