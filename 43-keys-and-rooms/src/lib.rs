use std::collections::HashSet;

pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let mut unlocked_rooms = HashSet::from([0]);
    let mut unvisited_rooms = vec![0];

    while let Some(room) = unvisited_rooms.pop() {
        let keys: HashSet<_> = rooms[room as usize].iter().cloned().collect();

        unvisited_rooms.extend(keys.difference(&unlocked_rooms));

        unlocked_rooms.extend(keys);
        if unlocked_rooms.len() == rooms.len() {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]));
    }

    #[test]
    fn case2() {
        assert!(!can_visit_all_rooms(vec![
            vec![1, 3],
            vec![3, 0, 1],
            vec![2],
            vec![0]
        ]));
    }
}
