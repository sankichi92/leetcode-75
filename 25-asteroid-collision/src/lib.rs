pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    asteroids.into_iter().fold(Vec::new(), |mut results, asteroid| {
        if asteroid < 0 {
            while let Some(previous) = results.pop() {
                match previous {
                    size if size > 0 && size > asteroid.abs() => {
                        results.push(previous);
                        break
                    }
                    size if size > 0 && size == asteroid.abs() => break,
                    size if size < 0 => results.push(asteroid),
                    _ => ()
                }
            }
        } else {
            results.push(asteroid);
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
}
