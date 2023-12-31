use std::collections::HashSet;

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut appeared_nums = HashSet::new();
    for num in nums {
        if !appeared_nums.remove(&num) {
            appeared_nums.insert(num);
        }
    }
    *appeared_nums.iter().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(single_number(vec![2, 2, 1]), 1)
    }

    #[test]
    fn case2() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4)
    }

    #[test]
    fn case3() {
        assert_eq!(single_number(vec![1]), 1)
    }
}
