pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    (((a | b) ^ c).count_ones() + (a & b & !c).count_ones()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(min_flips(2, 6, 5), 3)
    }

    #[test]
    fn case2() {
        assert_eq!(min_flips(4, 2, 7), 1)
    }

    #[test]
    fn case3() {
        assert_eq!(min_flips(1, 2, 3), 0)
    }

    #[test]
    fn failed_case1() {
        assert_eq!(min_flips(7, 7, 7), 0)
    }
}
