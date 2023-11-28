fn main() {}

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut results = Vec::with_capacity(candies.len());
    
    let max_candies = *candies.iter().max().unwrap();

    for candy in candies.iter() {
        results.push(candy + extra_candies >= max_candies);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = kids_with_candies(vec![2, 3, 5, 1, 3], 3);
        assert_eq!(result, vec![true, true, true, false, true]);
    }

    #[test]
    fn case2() {
        let result = kids_with_candies(vec![4, 2, 1, 1, 2], 1);
        assert_eq!(result, vec![true, false, false, false, false]);
    }

    #[test]
    fn case3() {
        let result = kids_with_candies(vec![12, 1, 12], 10);
        assert_eq!(result, vec![true, false, true]);
    }
}
