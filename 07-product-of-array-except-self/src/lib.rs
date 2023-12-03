pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let product_all: i32 = nums.iter().product();

    nums.iter()
        .enumerate()
        .map(|(i, num)| {
            if *num == 0 {
                nums.iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, num)| num)
                    .product()
            } else {
                product_all / num
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn case2() {
        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
