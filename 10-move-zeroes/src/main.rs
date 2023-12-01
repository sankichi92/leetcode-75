fn main() {}

pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut current = 0;
    for num in nums.clone() {
        if num == 0 {
            let num = nums.remove(current);
            nums.push(num);
        } else {
            current += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn case2() {
        let mut nums = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}
