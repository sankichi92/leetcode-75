pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    products.sort_unstable();

    let mut results = vec![];

    let mut current = "".to_string();
    for char in search_word.chars() {
        current.push(char);
        results.push(
            products
                .iter()
                .filter(|product| product.starts_with(&current))
                .take(3)
                .cloned()
                .collect(),
        );
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            suggested_products(
                vec![
                    "mobile".to_string(),
                    "mouse".to_string(),
                    "moneypot".to_string(),
                    "monitor".to_string(),
                    "mousepad".to_string()
                ],
                "mouse".to_string()
            ),
            vec![
                vec!["mobile", "moneypot", "monitor"],
                vec!["mobile", "moneypot", "monitor"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"]
            ]
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            suggested_products(vec!["havana".to_string()], "havana".to_string()),
            vec![
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"]
            ]
        )
    }
}
