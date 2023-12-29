pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut results = 0;
    let mut current = cost.len();

    loop {
        if current == 0 || current == 1 {
            return results;
        }

        if cost[current - 1] < cost[current - 2] {
            results += cost[current - 1];
            current -= 1
        } else {
            results += cost[current - 2];
            current -= 2
        }
    }
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
}
