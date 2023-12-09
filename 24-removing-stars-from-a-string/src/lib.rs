pub fn remove_stars(s: String) -> String {
    s.chars().fold(String::new(), |mut res, c| {
        if c == '*' {
            res.pop();
        } else {
            res.push(c);
        }
        res
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(remove_stars("leet**cod*e".to_string()), "lecoe".to_string());
    }

    #[test]
    fn case2() {
        assert_eq!(remove_stars("erase*****".to_string()), "".to_string());
    }
}
