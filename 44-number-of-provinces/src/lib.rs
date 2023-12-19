use std::collections::{HashMap, HashSet};

pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut city_to_province = HashMap::with_capacity(is_connected.len());
    
    for (i, row) in is_connected.iter().enumerate() {
        let province = *city_to_province.entry(i).or_insert(i);
        for (j, connected) in row.iter().enumerate().skip(i + 1) {
            if *connected == 1 {
                city_to_province.entry(j).or_insert(province);
            }
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
        assert_eq!(find_circle_num(vec![vec![1,1,0],vec![1,1,0],vec![0,0,1]]), 2);
    }

    #[test]
    fn case2() {
        assert_eq!(find_circle_num(vec![vec![1,0,0],vec![0,1,0],vec![0,0,1]]), 3);
    }
}
