pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    potions.sort_unstable();

    spells
        .into_iter()
        .map(|spell| {
            let spell = spell as i64;
            let idx = potions.partition_point(|&potion| (spell * potion as i64) < success);
            (potions.len() - idx) as i32
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
            vec![4, 0, 3]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
            vec![2, 0, 2]
        );
    }
}
