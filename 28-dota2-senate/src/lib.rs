pub fn predict_party_victory(senate: String) -> String {
    let (mut r, mut d) = senate.chars().fold((0, 0), |(mut r, mut d), senator| {
        match senator {
            'R' => r += 1,
            'D' => d += 1,
            _ => (),
        }
        (r, d)
    });

    for senator in senate.chars() {
        match senator {
            'R' => d -= 1,
            'D' => r -= 1,
            _ => (),
        }

        match r {
            r if r > d => return String::from("Radiant"),
            r if r < d => return String::from("Dire"),
            _ => ()
        }
    }

    panic!();
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
}
