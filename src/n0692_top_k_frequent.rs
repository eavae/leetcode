pub struct Solution {}

// submission codes start here
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    vec,
};

#[derive(Eq)]
struct Pair(String, u32);

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.1.cmp(&other.1);
        if ord == Ordering::Equal {
            self.0.cmp(&other.0).reverse()
        } else {
            ord
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1 && self.0 == other.0
    }
}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut map = HashMap::new();
        for word in words.into_iter() {
            map.entry(word).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut heap = BinaryHeap::new();
        for (k, v) in map.into_iter() {
            heap.push(Pair(k, v));
        }

        let mut res = Vec::new();
        let mut i = k;
        while let Some(Pair(k, _)) = heap.pop() {
            if i > 0 {
                res.push(k);
            } else {
                break;
            }
            i -= 1;
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
            vec![String::from("i"), String::from("love")],
            Solution::top_k_frequent(
                vec![
                    String::from("i"),
                    String::from("love"),
                    String::from("leetcode"),
                    String::from("i"),
                    String::from("love"),
                    String::from("coding")
                ],
                2
            )
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![
                String::from("the"),
                String::from("is"),
                String::from("sunny"),
                String::from("day")
            ],
            Solution::top_k_frequent(
                vec![
                    String::from("the"),
                    String::from("day"),
                    String::from("is"),
                    String::from("sunny"),
                    String::from("the"),
                    String::from("the"),
                    String::from("the"),
                    String::from("sunny"),
                    String::from("is"),
                    String::from("is")
                ],
                4
            )
        );
    }
}
