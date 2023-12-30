const MODULO: i64 = 1_000_000_007;

pub fn num_tilings(n: i32) -> i32 {
    let n = n as usize;

    let mut tilings = Vec::with_capacity(if n > 3 { n } else { 3 });
    tilings.push(1);
    tilings.push(2);
    tilings.push(5);

    for i in 3..n {
        tilings.push(((2 * tilings[i - 1] as i64 + tilings[i - 3] as i64) % MODULO) as i32);
    }

    tilings[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(num_tilings(3), 5);
    }

    #[test]
    fn case2() {
        assert_eq!(num_tilings(1), 1);
    }

    #[test]
    fn failed_case1() {
        assert_eq!(num_tilings(4), 11);
    }

    #[test]
    fn failed_case2() {
        assert_eq!(num_tilings(5), 24);
    }

    #[test]
    fn failed_case3() {
        assert_eq!(num_tilings(30), 312342182);
    }
}
