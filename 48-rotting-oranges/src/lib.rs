use std::collections::VecDeque;

pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut queue = VecDeque::new();

    for (i, row) in grid.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            if *val == 2 {
                *val = 1;
                queue.push_back((i, j, 0));
            }
        }
    }

    let bottom_index = grid.len() - 1;
    let right_index = grid[0].len() - 1;

    let mut max_min = 0;
    while let Some((i, j, min)) = queue.pop_front() {
        if grid[i][j] != 1 {
            continue;
        }

        grid[i][j] = 2;
        max_min = max_min.max(min);

        let min = min + 1;
        if i > 0 && grid[i - 1][j] == 1 {
            queue.push_back((i - 1, j, min));
        }
        if i < bottom_index && grid[i + 1][j] == 1 {
            queue.push_back((i + 1, j, min));
        }
        if j > 0 && grid[i][j - 1] == 1 {
            queue.push_back((i, j - 1, min));
        }
        if j < right_index && grid[i][j + 1] == 1 {
            queue.push_back((i, j + 1, min));
        }
    }

    if grid.iter().any(|row| row.iter().any(|val| *val == 1)) {
        -1
    } else {
        max_min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]]),
            4
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            oranges_rotting(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]]),
            -1
        );
    }

    #[test]
    fn case3() {
        assert_eq!(oranges_rotting(vec![vec![0, 2]]), 0);
    }
}
