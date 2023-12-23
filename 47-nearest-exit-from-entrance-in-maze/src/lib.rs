use std::collections::VecDeque;

pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    let entrance = [entrance[0] as usize, entrance[1] as usize];
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    visited[entrance[0]][entrance[1]] = true;

    let bottom_exit = maze.len() - 1;
    let right_exit = maze[0].len() - 1;

    let mut queue = VecDeque::new();
    if entrance[0] > 0 {
        queue.push_back((entrance[0] - 1, entrance[1], 1));
    }
    if entrance[0] < bottom_exit {
        queue.push_back((entrance[0] + 1, entrance[1], 1));
    }
    if entrance[1] > 0 {
        queue.push_back((entrance[0], entrance[1] - 1, 1));
    }
    if entrance[1] < right_exit {
        queue.push_back((entrance[0], entrance[1] + 1, 1));
    }

    while let Some((i, j, step)) = queue.pop_front() {
        if visited[i][j] {
            continue;
        }

        visited[i][j] = true;

        if maze[i][j] == '+' {
            continue;
        }

        if i == 0 || i == bottom_exit || j == 0 || j == right_exit {
            return step;
        }

        let step = step + 1;
        queue.push_back((i - 1, j, step));
        queue.push_back((i + 1, j, step));
        queue.push_back((i, j - 1, step));
        queue.push_back((i, j + 1, step));
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            nearest_exit(
                vec![
                    vec!['+', '+', '.', '+'],
                    vec!['.', '.', '.', '+'],
                    vec!['+', '+', '+', '.']
                ],
                vec![1, 2]
            ),
            1
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            nearest_exit(
                vec![
                    vec!['+', '+', '+'],
                    vec!['.', '.', '.'],
                    vec!['+', '+', '+']
                ],
                vec![1, 0]
            ),
            2
        )
    }

    #[test]
    fn case3() {
        assert_eq!(nearest_exit(vec![vec!['.', '+']], vec![0, 0]), -1)
    }
}
