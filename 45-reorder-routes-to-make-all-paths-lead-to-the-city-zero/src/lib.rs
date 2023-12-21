pub fn min_reorder(n: i32, mut connections: Vec<Vec<i32>>) -> i32 {
    let mut results = 0;
    let mut connected = vec![false; n as usize];
    connected[0] = true;

    while !connections.is_empty() {
        connections.retain(|conn| {
            if connected[conn[0] as usize] {
                results += 1;
                connected[conn[1] as usize] = true;
                false
            } else if connected[conn[1] as usize] {
                connected[conn[0] as usize] = true;
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
