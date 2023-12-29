pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let (mut left, mut right) = (1, *piles.iter().max().unwrap());
    'bsearch: loop {
        let mid = left + (right - left) / 2;

        let mut hours = 0;
        for pile in piles.iter() {
            hours += (*pile as f64 / mid as f64).ceil() as i32;
            if hours > h {
                left = mid + 1;
                continue 'bsearch;
            }
        }

        let mut hours = 0;
        for pile in piles.iter() {
            hours += (*pile as f64 / (mid - 1) as f64).ceil() as i32;
            if hours > h {
                return mid;
            }
        }
        right = mid - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4)
    }

    #[test]
    fn case2() {
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30)
    }

    #[test]
    fn case3() {
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23)
    }

    #[test]
    fn failed_case1() {
        assert_eq!(min_eating_speed(vec![312884470], 312884470), 1)
    }
}
