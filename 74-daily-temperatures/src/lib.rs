pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut results = vec![0; temperatures.len()];
    
    let mut no_warmer_days = vec![(0, temperatures[0])];

    for (current_day, current_temperature) in temperatures.into_iter().enumerate().skip(1) {
        while let Some((day, temperature)) = no_warmer_days.pop()  {
            if current_temperature > temperature {
                results[day] = (current_day - day) as i32;
            } else {
                no_warmer_days.push((day, temperature));
                break;
            }
        }

        no_warmer_days.push((current_day, current_temperature));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
    }

    #[test]
    fn case3() {
        assert_eq!(daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);
    }
}
