use std::collections::HashSet;

pub fn min_reorder(_n: i32, mut connections: Vec<Vec<i32>>) -> i32 {
    let mut results = 0;
    let mut stack = vec![HashSet::from([0])];

    while let Some(targets) = stack.pop() {
        let mut next_targets = HashSet::new();
        
        connections.retain(|conn| {
            if targets.contains(&conn[0]) {
                results += 1;
                next_targets.insert(conn[1]);
                false
            } else if targets.contains(&conn[1]) {
                next_targets.insert(conn[0]);
                false
            } else {
                true
            }
        });
        
        if !next_targets.is_empty() {
            stack.push(next_targets);
        }
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
