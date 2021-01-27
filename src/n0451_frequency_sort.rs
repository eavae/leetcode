pub struct Solution {}

// submission codes start here
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    vec,
};

#[derive(Eq)]
struct Pair(char, u32);

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map = HashMap::new();
        for c in s.chars().into_iter() {
            map.entry(c).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut heap = BinaryHeap::new();
        for (k, v) in map.into_iter() {
            heap.push(Pair(k, v));
        }

        let mut res = String::new();
        while let Some(Pair(k, mut n)) = heap.pop() {
            while n > 0 {
                res.push(k);
                n -= 1;
            }
        }

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            String::from("eetr"),
            Solution::frequency_sort(String::from("tree"))
        );
        assert_eq!(
            String::from("cccaaa"),
            Solution::frequency_sort(String::from("cccaaa"))
        );
        assert_eq!(
            String::from("bbaA"),
            Solution::frequency_sort(String::from("Aabb"))
        );
    }
}
