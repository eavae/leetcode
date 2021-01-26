/**
 * [90] Subsets II
 *
 * Given a collection of integers that might contain duplicates, nums, return all possible subsets (the power set).
 *
 * Note: The solution set must not contain duplicate subsets.
 *
 * Example:
 *
 *
 * Input: [1,2,2]
 * Output:
 * [
 *   [2],
 *   [1],
 *   [1,2,2],
 *   [2,2],
 *   [1,2],
 *   []
 * ]
 *
 *
 */
pub struct Solution {}

// submission codes start here

/*
count the repeats of each number,
then in backtracking, each number can be picked up for 0..repeat times

using BTreeMap to preserve order (easy for test)
*/

use std::ops::Range;
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut none_dup_range: Range<usize> = 0..1;
        let mut prev_num: Option<&i32> = None;
        res.push(vec![]);

        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        for n in sorted_nums.iter() {
            let prev_subset_len = res.len();
            let is_duplicated = if let Some(prev) = prev_num {
                prev == n
            } else {
                false
            };
            let r = if is_duplicated {
                none_dup_range
            } else {
                0..prev_subset_len
            };

            let mut new_vec = res[r].to_vec();
            for v in new_vec.iter_mut() {
                v.push(*n);
            }
            res.append(&mut new_vec);

            prev_num = Some(n);
            none_dup_range = prev_subset_len..res.len();
        }

        res
    }
}

// use std::collections::BTreeMap;
// impl Solution {
//     pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
//         let mut res = Vec::new();
//         let nums = nums
//             .into_iter()
//             .fold(BTreeMap::new(), |mut map, v| {
//                 *map.entry(v).or_insert(0) += 1;
//                 map
//             })
//             .into_iter()
//             .collect::<Vec<(i32, i32)>>();
//         Solution::backtrack(0, vec![], &nums, &mut res);
//         res
//     }

//     fn backtrack(
//         start: usize,
//         mut curr: Vec<i32>,
//         nums: &Vec<(i32, i32)>,
//         result: &mut Vec<Vec<i32>>,
//     ) {
//         if start >= nums.len() {
//             result.push(curr);
//             return;
//         }
//         for repeat in 0..nums[start].1 + 1 {
//             let mut inner = curr.clone();
//             for _ in 0..repeat {
//                 inner.push(nums[start].0);
//             }
//             Solution::backtrack(start + 1, inner, nums, result);
//         }
//     }
// }

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_90() {
        assert_eq!(
            Solution::subsets_with_dup(vec![1, 2, 2]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![2, 2],
                vec![1, 2, 2],
            ]
        );
        assert_eq!(
            Solution::subsets_with_dup(vec![4, 4, 4, 1, 4]),
            vec![
                vec![],
                vec![1],
                vec![4],
                vec![1, 4],
                vec![4, 4],
                vec![1, 4, 4],
                vec![4, 4, 4],
                vec![1, 4, 4, 4],
                vec![4, 4, 4, 4],
                vec![1, 4, 4, 4, 4],
            ]
        );
        assert_eq!(Solution::subsets_with_dup(vec![1]), vec![vec![], vec![1],]);
        assert_eq!(Solution::subsets_with_dup(vec![]), vec![vec![],]);
    }
}
