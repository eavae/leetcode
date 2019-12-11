/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.
 *
 * Example:
 *
 * Input: "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut start, mut max) = (0, 0);
        for (index, character) in s.chars().enumerate() {
            if let Some(i) = &s[start..index].find(character) {
                start = start + *i + 1;
                // println!("start: {}, index: {}", start, i);
            }
            let len = index - start + 1;
            max = if len > max { len } else { max };
            // println!("{} is {}", index, character);
        }
        max as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }
}
