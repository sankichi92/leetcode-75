pub fn unique_paths(m: i32, n: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    let mut grid = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            grid[i][j] = match (i, j) {
                (0, _) => 1,
                (_, 0) => 1,
                _ => grid[i - 1][j] + grid[i][j - 1],
            }
        }
    }

    grid[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(unique_paths(3, 7), 28);
    }

    #[test]
    fn case2() {
        assert_eq!(unique_paths(3, 2), 3);
    }

    #[test]
    fn failed_case1() {
        assert_eq!(unique_paths(1, 1), 1);
    }
}
