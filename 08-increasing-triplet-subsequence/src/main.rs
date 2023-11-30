fn main() {}

pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut first_candidate = nums[0];
    let mut second_candidate = None;

    for num in nums.iter().skip(1) {
        if let Some(second) = second_candidate {
            if *num > second {
                return true;
            } else {
                second_candidate = Some(*num);
            }
        }

        if *num > first_candidate {
            second_candidate = Some(*num);
        } else {
            first_candidate = *num
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(increasing_triplet(vec![1, 2, 3, 4, 5]), true);
    }

    #[test]
    fn case2() {
        assert_eq!(increasing_triplet(vec![5, 4, 3, 2, 1]), false);
    }

    #[test]
    fn case3() {
        assert_eq!(increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
    }
}
