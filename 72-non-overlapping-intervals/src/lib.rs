pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_unstable_by_key(|interval| interval[1]);

    let mut removed = 0;
    let mut current_end = intervals[0][1];

    for interval in intervals.iter().skip(1) {
        if interval[0] < current_end {
            removed += 1;
        } else {
            current_end = interval[1];
        }
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

    #[test]
    fn failed_case1() {
        assert_eq!(
            erase_overlap_intervals(vec![vec![1, 2], vec![1, 3], vec![1, 4]]),
            2
        );
    }

    #[test]
    fn failed_case2() {
        assert_eq!(
            erase_overlap_intervals(vec![
                vec![0, 2],
                vec![1, 3],
                vec![1, 3],
                vec![2, 4],
                vec![3, 5],
                vec![3, 5],
                vec![4, 6]
            ]),
            4
        );
    }

    #[test]
    fn failed_case3() {
        assert_eq!(
            erase_overlap_intervals(vec![
                vec![0, 2],
                vec![1, 3],
                vec![2, 4],
                vec![3, 5],
                vec![4, 6]
            ]),
            2
        );
    }
}
