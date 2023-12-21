pub fn min_reorder(_n: i32, mut connections: Vec<Vec<i32>>) -> i32 {
    let mut results = 0;
    let mut stack = vec![0];

    while let Some(target) = stack.pop() {
        connections.retain(|conn| {
            if conn[0] == target {
                results += 1;
                stack.push(conn[1]);
                false
            } else if conn[1] == target {
                stack.push(conn[0]);
                false
            } else {
                true
            }
        });
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            min_reorder(
                6,
                vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]
            ),
            3
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]),
            2
        )
    }

    #[test]
    fn case3() {
        assert_eq!(min_reorder(3, vec![vec![1, 0], vec![2, 0]]), 0)
    }
}
