pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut results = vec![];
    let mut candidate: Vec<_> = (1..=k).collect();

    'main: loop {
        if candidate.iter().sum::<i32>() == n {
            results.push(candidate.clone());
        }

        for i in (0..k).rev() {
            if candidate[i as usize] < 9 + k - 1 - i {
                candidate[i as usize] += 1;
                for j in i + 1..k {
                    candidate[j as usize] = candidate[i as usize] + j - i;
                }
                continue 'main;
            }
        }

        break;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(combination_sum3(3, 7), vec![vec![1, 2, 4]]);
    }

    #[test]
    fn case2() {
        assert_eq!(
            combination_sum3(3, 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
    }

    #[test]
    fn case3() {
        assert_eq!(combination_sum3(4, 1), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn failed_case1() {
        assert_eq!(
            combination_sum3(3, 15),
            vec![
                vec![1, 5, 9],
                vec![1, 6, 8],
                vec![2, 4, 9],
                vec![2, 5, 8],
                vec![2, 6, 7],
                vec![3, 4, 8],
                vec![3, 5, 7],
                vec![4, 5, 6]
            ]
        );
    }
}
