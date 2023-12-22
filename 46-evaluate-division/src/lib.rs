use std::collections::HashMap;

pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let mut equation_map = HashMap::new();

    for (equation, value) in equations.into_iter().zip(values.into_iter()) {
        let result_map = equation_map
            .entry(equation[0].clone())
            .or_insert(HashMap::new());
        result_map.insert(equation[1].clone(), value);

        let result_map = equation_map
            .entry(equation[1].clone())
            .or_insert(HashMap::new());
        result_map.insert(equation[0].clone(), 1.0 / value);
    }

    fn inner_calc_equation(
        equation_map: &HashMap<String, HashMap<String, f64>>,
        query: &[String],
        route: &mut Vec<String>,
        before: f64,
    ) -> Option<f64> {
        if let Some(next) = equation_map.get(&query[0]) {
            route.push(query[0].clone());

            if let Some(val) = next.get(&query[1]) {
                return Some(before * *val);
            } else {
                for (var, val) in next.iter() {
                    if route.contains(var) {
                        continue;
                    }

                    if let Some(result) = inner_calc_equation(
                        equation_map,
                        &[var.clone(), query[1].clone()],
                        route,
                        before * *val,
                    ) {
                        return Some(result);
                    }
                }
            }
        }

        None
    }

    queries
        .into_iter()
        .map(|query| inner_calc_equation(&equation_map, &query, &mut vec![], 1.0).unwrap_or(-1.0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            calc_equation(
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("c")]
                ],
                vec![2.0, 3.0],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("a"), String::from("e")],
                    vec![String::from("a"), String::from("a")],
                    vec![String::from("x"), String::from("x")]
                ]
            ),
            vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000]
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            calc_equation(
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("c")],
                    vec![String::from("bc"), String::from("cd")]
                ],
                vec![1.5, 2.5, 5.0],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("c"), String::from("b")],
                    vec![String::from("bc"), String::from("cd")],
                    vec![String::from("cd"), String::from("bc")]
                ]
            ),
            vec![3.75000, 0.40000, 5.00000, 0.20000]
        )
    }

    #[test]
    fn case3() {
        assert_eq!(
            calc_equation(
                vec![vec![String::from("a"), String::from("b")]],
                vec![0.5],
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("x"), String::from("y")]
                ]
            ),
            vec![0.50000, 2.00000, -1.00000, -1.00000]
        )
    }
}
