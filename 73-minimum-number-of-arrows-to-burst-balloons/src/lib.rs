pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_unstable_by_key(|point| point[0]);

    let mut arrows = 1;
    let mut current_end = points[0][1];

    for point in points.iter().skip(1) {
        if point[0] <= current_end {
            current_end = current_end.min(point[1]);
        } else {
            arrows += 1;
            current_end = point[1];
        }
    }

    arrows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
            2
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
            4
        )
    }

    #[test]
    fn case3() {
        assert_eq!(
            find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
            2
        )
    }

    #[test]
    fn failed_case1() {
        assert_eq!(
            find_min_arrow_shots(vec![
                vec![9, 12],
                vec![1, 10],
                vec![4, 11],
                vec![8, 12],
                vec![3, 9],
                vec![6, 9],
                vec![6, 7]
            ]),
            2
        )
    }
}
