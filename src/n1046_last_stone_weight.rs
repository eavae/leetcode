pub struct Solution {}

// submission codes start here
use std::collections::BinaryHeap;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::from(stones);

        loop {
            match (heap.pop(), heap.pop()) {
                (Some(a), Some(b)) => {
                    if a != b {
                        heap.push(a - b);
                    }
                }
                (Some(a), None) => return a,
                (_, _) => return 0,
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
    }
}
