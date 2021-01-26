use std::collections::HashSet;

/**
 * [217] Contains Duplicate
 *
 * Given an array of integers, find if the array contains any duplicates.
 *
 * Your function should return true if any value appears at least twice in the array, and it should return false if every element is distinct.
 *
 * Example 1:
 *
 *
 * Input: [1,2,3,1]
 * Output: true
 *
 * Example 2:
 *
 *
 * Input: [1,2,3,4]
 * Output: false
 *
 * Example 3:
 *
 *
 * Input: [1,1,1,3,3,4,3,2,4,2]
 * Output: true
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return false;
        }

        let mut s: HashSet<i32> = HashSet::new();
        for n in nums.iter() {
            match s.contains(n) {
                true => return true,
                false => {
                    s.insert(*n);
                }
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_217() {
        assert_eq!(Solution::contains_duplicate(vec![1]), false);
        assert_eq!(Solution::contains_duplicate(vec![]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    }
}
