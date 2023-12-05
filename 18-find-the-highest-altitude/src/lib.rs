pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut highest = 0;
    let mut altitude = 0;

    for net in gain {
        altitude += net;
        highest = highest.max(altitude);
    }

    highest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
