pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut overlapped_counts = vec![0; intervals.len()];
    let mut overlapped_indices = vec![vec![]; intervals.len()];

    for (i, interval1) in intervals.iter().enumerate() {
        for (j, interval2) in intervals.iter().enumerate().filter(|(j, _)| *j != i) {
            if (interval1[0] < interval2[0] && interval2[0] < interval1[1])
                || (interval1[0] < interval2[1] && interval2[1] < interval1[1])
                || (interval1[0] == interval2[0] && interval1[1] == interval2[1])
            {
                overlapped_counts[i] += 1;
                overlapped_indices[i].push(j);
            }
        }
    }

    let mut removed = 0;
    loop {
        let (max_idx, count) = overlapped_counts
            .iter_mut()
            .enumerate()
            .max_by(|(_, cnt1), (_, cnt2)| cnt1.cmp(cnt2))
            .unwrap();

        if *count == 0 {
            break;
        }

        *count = 0;
        for idx in overlapped_indices[max_idx].iter() {
            overlapped_counts[*idx] -= 1;
        }

        removed += 1
    }

    removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            2
        );
    }

    #[test]
    fn case3() {
        assert_eq!(erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]), 0);
    }
}
