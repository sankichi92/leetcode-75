// prices = [1,3,2,8,4,9], fee = 2
//    1  3  2  8  4  9
// _ [0, 0, 0, 0, 0, 0]
// 1 [_, 0, 0, 5, 5, 6]
// 3 [_, _, 0, 5, 5, 6]
// 2 [_, _, _, 5, 5, 6]
// 8 [_, _, _, _, 5, 6]
// 4 [_, _, _, _, _, 8]
pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let mut last_buy = -prices[0];
    let mut profit = 0;

    for price in prices.into_iter().skip(1) {
        let current_profit = profit;
        profit = profit.max(last_buy + price - fee);
        last_buy = last_buy.max(current_profit - price);
    }
    profit
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
