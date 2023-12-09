use std::collections::HashMap;

pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let size = grid.len();

    let mut nums_to_count =
        grid.clone()
            .into_iter()
            .fold(HashMap::with_capacity(size), |mut map, row| {
                let (r, _) = map.entry(row).or_insert((0, 0));
                *r += 1;
                map
            });

    for i in 0..size {
        let col: Vec<_> = grid.iter().map(|row| row[i]).collect();
        if let Some((_, c)) = nums_to_count.get_mut(&col) {
            *c += 1;
        }
    }

    nums_to_count.values().fold(0, |mut sum, (r, c)| {
        sum += r * c;
        sum
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]),
            1
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 5],
                vec![2, 4, 2, 2],
                vec![2, 4, 2, 2]
            ]),
            3
        );
    }
}
