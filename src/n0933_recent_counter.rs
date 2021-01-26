pub struct Solution {}

// submission codes start here
use std::collections::VecDeque;
struct RecentCounter {
    q: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter { q: VecDeque::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.q.push_back(t);

        let cursor = t - 3000;
        while let Some(&v) = self.q.get(0) {
            if v < cursor {
                self.q.pop_front();
            } else {
                break;
            }
        }
        self.q.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut counter = RecentCounter::new();
        assert_eq!(1, counter.ping(1));
        assert_eq!(2, counter.ping(100));
        assert_eq!(3, counter.ping(3001));
        assert_eq!(3, counter.ping(3002));
    }
}
