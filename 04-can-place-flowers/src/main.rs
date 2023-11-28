fn main() {}

pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut capacity = 0;

    let mut previous = 0;
    for (i, current) in flowerbed.iter().enumerate() {
        let next = flowerbed.get(i + 1).unwrap_or(&0);

        if previous == 0 && *current == 0 && *next == 0 {
            capacity += 1;
            if capacity >= n {
                return true;
            }
            previous = 1;
        } else {
            previous = *current;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = can_place_flowers(vec![1, 0, 0, 0, 1], 1);
        assert_eq!(result, true);
    }

    #[test]
    fn case2() {
        let result = can_place_flowers(vec![1, 0, 0, 0, 1], 2);
        assert_eq!(result, false);
    }
}
