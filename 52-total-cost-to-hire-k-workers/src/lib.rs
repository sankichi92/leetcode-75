use std::{cmp::Reverse, collections::BinaryHeap};

pub fn total_cost(mut costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    let k = k as usize;
    let candidates = candidates as usize;

    let mut total_cost: i64 = 0;

    let mut left_candidates: BinaryHeap<_> =
        costs.iter().take(candidates).map(|c| Reverse(*c)).collect();
    let mut right_candidates: BinaryHeap<_> = costs
        .iter()
        .rev()
        .take(candidates.min(costs.len() - candidates))
        .map(|c| Reverse(*c))
        .collect();

    for _ in 0..k {
        let left_min = left_candidates.peek().unwrap_or(&Reverse(i32::MAX)).0;
        let right_min = right_candidates.peek().unwrap_or(&Reverse(i32::MAX)).0;

        if left_min <= right_min {
            total_cost += left_candidates.pop().unwrap().0 as i64;

            if costs.len() > candidates * 2 {
                left_candidates.push(Reverse(costs.remove(candidates)));
            }
        } else {
            total_cost += right_candidates.pop().unwrap().0 as i64;

            if costs.len() > candidates * 2 {
                right_candidates.push(Reverse(costs.remove(costs.len() - candidates - 1)));
            }
        }
    }

    total_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4), 11);
    }

    #[test]
    fn case2() {
        assert_eq!(total_cost(vec![1, 2, 4, 1], 3, 3), 4);
    }

    #[test]
    fn failed_case1() {
        total_cost(vec![57, 33, 26, 76, 14, 67, 24, 90, 72, 37, 30], 11, 2);
    }
}
