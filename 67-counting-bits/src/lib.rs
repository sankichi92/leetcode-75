pub fn count_bits(n: i32) -> Vec<i32> {
    (0..=n)
        .map(|i| format!("{i:b}").chars().filter(|&c| c == '1').count() as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn case2() {
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
