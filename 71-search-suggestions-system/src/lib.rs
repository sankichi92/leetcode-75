use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    children: HashMap<char, Box<Trie>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: HashMap::new(),
        }
    }

    fn build(words: Vec<String>) -> Self {
        let mut children = HashMap::new();
        for word in words {
            let mut chars = word.chars();
            let mut current = children
                .entry(chars.next().unwrap())
                .or_insert(Box::new(Trie::new()));
            for char in chars {
                current = current
                    .children
                    .entry(char)
                    .or_insert(Box::new(Trie::new()));
            }
        }
        Trie { children }
    }

    fn words(&self, mut prefix: String) -> Vec<String> {
        if self.children.is_empty() {
            return vec![prefix.to_string()];
        }

        let mut results = vec![];
        for (char, trie) in self.children.iter() {
            prefix.push(*char);
            let mut words = trie.words(prefix.clone());
            words.sort();
            results.extend(words.into_iter().take(3));
        }

        results
    }
}

pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    let mut results = vec![];

    let trie = Trie::build(products);
    println!("{:?}", trie);

    let mut chars = search_word.chars();
    let first_char = chars.next().unwrap();
    let mut current_trie = trie.children.get(&first_char);
    let mut current_word = first_char.to_string();

    for char in chars {
        if let Some(trie) = current_trie {
            results.push(trie.words(current_word.clone()));
            current_trie = trie.children.get(&char);
        } else {
            results.push(vec![]);
        }
        current_word.push(char);
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
