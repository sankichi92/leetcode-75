pub fn predict_party_victory(senate: String) -> String {
    let mut senate = senate;

    loop {
        let mut r = 0;
        let mut d = 0;

        let current_senate = senate.clone();
        senate.clear();

        for senator in current_senate.chars() {
            match senator {
                'R' => {
                    if d > 0 {
                        d -= 1;
                    } else {
                        r += 1;
                        senate.push(senator);
                    }
                }
                'D' => {
                    if r > 0 {
                        r -= 1;
                    } else {
                        d += 1;
                        senate.push(senator);
                    }
                }
                _ => panic!(),
            }
        }

        let current_senate = senate.clone();
        senate.clear();

        for senator in current_senate.chars() {
            match senator {
                'R' => {
                    if d > 0 {
                        d -= 1;
                    } else {
                        senate.push(senator);
                    }
                }
                'D' => {
                    if r > 0 {
                        r -= 1;
                    } else {
                        senate.push(senator);
                    }
                }
                _ => panic!(),
            }
        }

        match senate.chars().filter(|&senator| senator == 'R').count() {
            r if r == senate.len() => return String::from("Radiant"),
            0 => return String::from("Dire"),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            predict_party_victory("RD".to_string()),
            "Radiant".to_string()
        )
    }

    #[test]
    fn case2() {
        assert_eq!(predict_party_victory("RDD".to_string()), "Dire".to_string())
    }

    #[test]
    fn failed_case1() {
        assert_eq!(
            predict_party_victory("DRDRR".to_string()),
            "Dire".to_string()
        )
    }

    #[test]
    fn failed_case2() {
        assert_eq!(predict_party_victory("D".to_string()), "Dire".to_string())
    }
}
