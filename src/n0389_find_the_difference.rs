pub struct Solution {}

// submission codes start here
use std::{char, iter};
impl Solution {
    pub fn find_the_difference(a: String, b: String) -> char {
        let mut total: i32 = 0;
        for (r, l) in b.chars().zip(a.chars().chain(iter::repeat('\0'))) {
            total += ((r as i32) - (l as i32))
        }
        char::from_u32(total as u32).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_389() {
        assert_eq!(
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
            'e'
        );
        assert_eq!(
            Solution::find_the_difference("".to_string(), "y".to_string()),
            'y'
        );
        assert_eq!(
            Solution::find_the_difference("a".to_string(), "aa".to_string()),
            'a'
        );
        assert_eq!(
            Solution::find_the_difference("ae".to_string(), "aea".to_string()),
            'a'
        );
    }
}
