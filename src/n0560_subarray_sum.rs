pub struct Solution {}

// submission codes start here
// 暴力法
// impl Solution {
//     pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
//         let mut count = 0;
//         let mut sum = 0;
//         for end in 0..nums.len() {
//             sum = 0;
//             for start in (0..=end).rev() {
//                 sum += nums[start];
//                 if (sum == k) {
//                     count += 1;
//                 }
//             }
//         }
//         count
//     }
// }

use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut sum = 0;
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        map.insert(0, 1);

        for &n in nums.iter() {
            sum += n;
            if map.contains_key(&(sum - k)) {
                count += map.get(&(sum - k)).unwrap();
            }
            *map.entry(sum).and_modify(|v| *v += 1).or_insert(1);
        }

        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
        assert_eq!(2, Solution::subarray_sum(vec![1, 2, 3], 3));
    }
}
