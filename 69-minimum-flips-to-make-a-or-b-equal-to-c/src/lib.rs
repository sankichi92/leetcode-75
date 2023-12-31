pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    // println!("a: {a:b}, b: {b:b}, a|b: {:b}, c: {c:b}", a | b);
    (((a | b) ^ c).count_ones() + (a & b).count_ones()) as i32
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
}
