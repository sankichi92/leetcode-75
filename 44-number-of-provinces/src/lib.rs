use std::collections::{HashMap, HashSet};

pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut city_to_province = HashMap::with_capacity(is_connected.len());

    for (i, row) in is_connected.iter().enumerate() {
        let connected_cities: Vec<_> = row
            .iter()
            .enumerate()
            .filter(|(_, &val)| val == 1)
            .map(|(j, _)| j)
            .collect();

        let provinces: Vec<_> = connected_cities
            .iter()
            .filter_map(|city| city_to_province.get(city).copied())
            .collect();
        let integrated_province = *provinces.iter().min().unwrap_or(&i);

        if provinces.len() > 1 {
            for (_, p) in city_to_province.iter_mut() {
                if provinces.contains(p) {
                    *p = integrated_province
                }
            }
        }

        for city in connected_cities {
            city_to_province.insert(city, integrated_province);
        }
    }

    let provinces: HashSet<_> = city_to_province.values().collect();
    provinces.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
            2
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }

    #[test]
    fn failed_case1() {
        assert_eq!(
            find_circle_num(vec![
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 0],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 1]
            ]),
            1
        );
    }

    #[test]
    fn failed_case2() {
        assert_eq!(
            find_circle_num(vec![
                vec![1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0],
                vec![1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1],
                vec![0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
            ]),
            3
        );
    }
}
