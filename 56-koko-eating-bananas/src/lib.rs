pub fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
    piles.sort_unstable();
    let candidates: Vec<_> = (1..*piles.last().unwrap()).collect();
    candidates.partition_point(|candidate| {
        let hours: i32 = piles.iter().map(|pile| (*pile as f64 / *candidate as f64).ceil() as i32).sum();
        hours > h
    }) as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4)
    }

    #[test]
    fn case2() {
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30)
    }

    #[test]
    fn case3() {
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23)
    }
}
