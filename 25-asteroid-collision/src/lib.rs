pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    asteroids.iter().fold(Vec::new(), |mut results, asteroid| {
        if *asteroid < 0 {
            loop {
                match results.pop() {
                    Some(previous) if previous > 0 && previous > asteroid.abs() => {
                        results.push(previous);
                        break;
                    }
                    Some(previous) if previous > 0 && previous == asteroid.abs() => break,
                    Some(previous) if previous < 0 => {
                        results.extend([previous, *asteroid]);
                        break;
                    }
                    Some(_) => (),
                    None => {
                        results.push(*asteroid);
                        break;
                    }
                }
            }
        } else {
            results.push(*asteroid);
        }
        results
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(asteroid_collision(vec![5, 10, -5]), vec![5, 10])
    }

    #[test]
    fn case2() {
        assert_eq!(asteroid_collision(vec![8, -8]), vec![])
    }

    #[test]
    fn case3() {
        assert_eq!(asteroid_collision(vec![10, 2, -5]), vec![10])
    }

    #[test]
    fn failed_case1() {
        assert_eq!(asteroid_collision(vec![-2, -1, 1, 2]), vec![-2, -1, 1, 2])
    }

    #[test]
    fn failed_case2() {
        assert_eq!(asteroid_collision(vec![1, -2, -2, -2]), vec![-2, -2, -2])
    }
}
