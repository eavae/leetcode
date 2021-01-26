/**
 * [78] Subsets
 *
 * Given a set of distinct integers, nums, return all possible subsets (the power set).
 *
 * Note: The solution set must not contain duplicate subsets.
 *
 * Example:
 *
 *
 * Input: nums = [1,2,3]
 * Output:
 * [
 *   [3],
 *   [1],
 *   [2],
 *   [1,2,3],
 *   [1,3],
 *   [2,3],
 *   [1,2],
 *   []
 * ]
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        res.push(vec![]);

        for n in nums.iter() {
            let mut new_vec = res.clone();
            for v in new_vec.iter_mut() {
                v.push(*n);
            }
            res.append(&mut new_vec);
        }

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_78() {
        assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
        assert_eq!(Solution::subsets(vec![1]), vec![vec![], vec![1]]);
        assert_eq!(
            Solution::subsets(vec![1, 2]),
            vec![vec![], vec![1], vec![2], vec![1, 2]]
        );
    }
}
