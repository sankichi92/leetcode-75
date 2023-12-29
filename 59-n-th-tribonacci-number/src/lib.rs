pub fn tribonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => tribonacci(n - 1) + tribonacci(n - 2) + tribonacci(n - 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(tribonacci(4), 4)
    }

    #[test]
    fn case2() {
        assert_eq!(tribonacci(25), 1389537)
    }
}
