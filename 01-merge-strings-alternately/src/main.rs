fn main() {}

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::new();

    for (c1, c2) in word1.chars().into_iter().zip(word2.chars().into_iter()) {
        result.push(c1);
        result.push(c2);
    }

    if word1.len() == word2.len() {
        return result;
    }

    let additional_letters = if word1.len() > word2.len() {
        word1.chars().skip(word2.len())
    } else {
        word2.chars().skip(word1.len())
    };

    result.extend(additional_letters);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = merge_alternately(String::from("abc"), String::from("pqr"));
        assert_eq!(result, "apbqcr");
    }

    #[test]
    fn case2() {
        let result = merge_alternately(String::from("ab"), String::from("pqrs"));
        assert_eq!(result, "apbqrs");
    }

    #[test]
    fn case3() {
        let result = merge_alternately(String::from("abcd"), String::from("pq"));
        assert_eq!(result, "apbqcd");
    }
}
