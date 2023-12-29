pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut results = vec![];
    let mut candidate: Vec<_> = (1..=k).collect();
    
    'main: loop {
        if candidate.iter().sum::<i32>() == n {
            results.push(candidate.clone());
        }

        for i in (0..k).rev() {
            if candidate[i as usize] < 9 - i {
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
        let results = combination_sum3(3, 7);
        assert!(results.contains(&vec![1, 2, 4]), "{:?}", results);
    }

    #[test]
    fn case2() {
        let results = combination_sum3(3, 9);
        assert!(results.contains(&vec![1, 2, 6]), "{:?}", results);
        assert!(results.contains(&vec![1, 3, 5]), "{:?}", results);
        assert!(results.contains(&vec![2, 3, 4]), "{:?}", results);
    }

    #[test]
    fn case3() {
        let results = combination_sum3(4, 1);
        assert_eq!(results, Vec::<Vec<i32>>::new());
    }
}
