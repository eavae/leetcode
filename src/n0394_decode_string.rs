pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut str = String::new();
        let mut stack = Vec::new();

        let mut n = 0_usize;
        stack.push((1, String::new()));
        for c in s.chars() {
            if c.is_digit(10) {
                n = n * 10 + c.to_digit(10).unwrap() as usize;
            } else if c == '[' {
                stack.push((n, String::new()));
                n = 0;
            } else if c == ']' {
                let (count, chars) = stack.pop().unwrap();
                stack.last_mut().unwrap().1.push_str(&chars.repeat(count));
            } else {
                stack.last_mut().unwrap().1.push(c);
            }
        }
        stack.pop().unwrap().1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            String::from("aaabcbc"),
            Solution::decode_string(String::from("3[a]2[bc]"))
        );
        assert_eq!(
            String::from("accaccacc"),
            Solution::decode_string(String::from("3[a2[c]]"))
        );
        assert_eq!(
            String::from("abcabccdcdcdef"),
            Solution::decode_string(String::from("2[abc]3[cd]ef"))
        );
        assert_eq!(
            String::from("abccdcdcdxyz"),
            Solution::decode_string(String::from("abc3[cd]xyz"))
        );
        assert_eq!(
            String::from("aaaaaaaaaa"),
            Solution::decode_string(String::from("10[a]"))
        );
    }
}
