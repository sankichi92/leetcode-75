pub fn num_tilings(n: i32) -> i32 {
    let n = n as usize;

    let mut tilings = Vec::with_capacity(if n > 3 { n } else { 3 });
    tilings.push(1);
    tilings.push(2);
    tilings.push(5);

    for i in 3..n {
        tilings.push(tilings[i - 1] + tilings[i - 2] + 2 * tilings[i - 3]);
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
}
