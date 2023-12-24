use std::{collections::{BinaryHeap, HashSet}, cmp::Reverse};

const DEFAULTS: [i32; 5] = [1, 2, 3, 4, 5];

pub struct SmallestInfiniteSet {
    set: HashSet<i32>,
    heap: BinaryHeap<Reverse<i32>>,
}

impl SmallestInfiniteSet {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SmallestInfiniteSet {
            set: HashSet::from(DEFAULTS),
            heap: DEFAULTS.into_iter().map(Reverse).collect(),
        }
    }

    pub fn pop_smallest(&mut self) -> i32 {
        let smallest = self.heap.pop().unwrap().0;
        self.set.remove(&smallest);
        smallest
    }

    pub fn add_back(&mut self, num: i32) {
        if self.set.insert(num) {
            self.heap.push(Reverse(num));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut set = SmallestInfiniteSet::new();
        set.add_back(2);
        assert_eq!(set.pop_smallest(), 1);
        assert_eq!(set.pop_smallest(), 2);
        assert_eq!(set.pop_smallest(), 3);
        set.add_back(1);
        assert_eq!(set.pop_smallest(), 1);
        assert_eq!(set.pop_smallest(), 4);
        assert_eq!(set.pop_smallest(), 5);
    }
}
