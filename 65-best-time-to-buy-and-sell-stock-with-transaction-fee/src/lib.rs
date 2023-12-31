// prices = [1,3,2,8,4,9], fee = 2
//    1  3  2  8  4  9
// _ [0, 0, 0, 0, 0, 0]
// 1 [_, 0, 0, 5, 5, 6]
// 3 [_, _, 0, 5, 5, 6]
// 2 [_, _, _, 5, 5, 6]
// 8 [_, _, _, _, 5, 6]
// 4 [_, _, _, _, _, 8]
use std::cmp;

pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let days = prices.len();

    let mut profit_matrix = vec![vec![0; days]; days];

    for (buy_day, buy_price) in prices.iter().enumerate().take(days - 1) {
        let current_profit = profit_matrix[buy_day][buy_day];
        for (sell_day, sell_price) in prices.iter().enumerate().skip(buy_day + 1) {
            let profit = current_profit + sell_price - buy_price - fee;
            profit_matrix[buy_day + 1][sell_day] = cmp::max(
                profit,
                cmp::max(
                    profit_matrix[buy_day + 1][sell_day - 1],
                    profit_matrix[buy_day][sell_day],
                ),
            );
            // println!("{:?}", profit_matrix);
        }
    }

    profit_matrix[days - 1][days - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8)
    }

    #[test]
    fn case2() {
        assert_eq!(max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6)
    }
}
