use std::cmp;

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    fn _min_cost_climbing_stairs(costs: &Vec<i32>, current: usize, sum: i32) -> i32 {
        if current >= costs.len() {
            sum
        } else {
            let sum = sum + costs[current];
            cmp::min(
                _min_cost_climbing_stairs(costs, current + 1, sum),
                _min_cost_climbing_stairs(costs, current + 2, sum),
            )
        }
    }
    cmp::min(
        _min_cost_climbing_stairs(&cost, 0, 0),
        _min_cost_climbing_stairs(&cost, 1, 0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn case2() {
        assert_eq!(
            min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }

    #[test]
    fn failed_case1() {
        assert_eq!(min_cost_climbing_stairs(vec![0, 2, 2, 1]), 2);
    }
}
