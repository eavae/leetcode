pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut v = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut v);

        assert_eq!(v, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
