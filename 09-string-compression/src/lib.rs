pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut current_group = CharGroup::new(chars[0]);
    let mut results = String::new();

    for &c in chars.iter().skip(1) {
        if c == current_group.char {
            current_group.incr()
        } else {
            results.push_str(&current_group.to_string());
            current_group = CharGroup::new(c)
        }
    }
    results.push_str(&current_group.to_string());

    chars.clear();
    chars.extend(results.chars());

    chars.len() as i32
}

struct CharGroup {
    char: char,
    count: i32,
}

impl CharGroup {
    fn new(char: char) -> Self {
        return CharGroup { char, count: 1 };
    }

    fn incr(&mut self) {
        self.count += 1;
    }

    fn to_string(&self) -> String {
        if self.count > 1 {
            format!("{}{}", self.char, self.count)
        } else {
            self.char.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(compress(&mut chars), 6);
        assert_eq!(chars, vec!['a', '2', 'b', '2', 'c', '3']);
    }

    #[test]
    fn case2() {
        let mut chars = vec!['a'];
        assert_eq!(compress(&mut chars), 1);
        assert_eq!(chars, vec!['a']);
    }

    #[test]
    fn case3() {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        assert_eq!(compress(&mut chars), 4);
        assert_eq!(chars, vec!['a', 'b', '1', '2'])
    }
}
