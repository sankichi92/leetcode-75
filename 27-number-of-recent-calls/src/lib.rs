pub struct RecentCounter {
    requests: Vec<i32>,
}

impl RecentCounter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        RecentCounter {
            requests: Vec::new(),
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        let mut count = 1;
        for &request_t in self.requests.iter().rev() {
            if request_t >= t - 3000 {
                count += 1;
            } else {
                break;
            }
        }

        self.requests.push(t);

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut recent_counter = RecentCounter::new();
        assert_eq!(recent_counter.ping(1), 1);
        assert_eq!(recent_counter.ping(100), 2);
        assert_eq!(recent_counter.ping(3001), 3);
        assert_eq!(recent_counter.ping(3002), 3);
    }
}
