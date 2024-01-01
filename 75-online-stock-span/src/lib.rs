pub struct StockSpanner {
    prices: Vec<i32>,
}

impl StockSpanner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        StockSpanner { prices: vec![] }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        for prev_price in self.prices.iter().rev() {
            if price >= *prev_price {
                span += 1;
            } else {
                break;
            }
        }
        self.prices.push(price);
        span
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut stock_spanner = StockSpanner::new();
        assert_eq!(stock_spanner.next(100), 1);
        assert_eq!(stock_spanner.next(80), 1);
        assert_eq!(stock_spanner.next(60), 1);
        assert_eq!(stock_spanner.next(70), 2);
        assert_eq!(stock_spanner.next(60), 1);
        assert_eq!(stock_spanner.next(75), 4);
        assert_eq!(stock_spanner.next(85), 6);
    }
}
