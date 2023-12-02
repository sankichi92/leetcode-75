fn main() {}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;

    for (i, val1) in height.iter().enumerate() {
        let min_len = max_area / val1;

        for (j, val2) in height.iter().skip(i + 1 + min_len as usize).enumerate() {
            let area = val1.min(val2) * (min_len + j as i32 + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn case2() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
