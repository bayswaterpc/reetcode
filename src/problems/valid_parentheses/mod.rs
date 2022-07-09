use super::Solution;
use std::collections::HashMap;

/// [Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)
impl Solution {
    //Below required for leetcode
    //pub fn is_valid(s: String) -> bool {
    pub fn valid_parentheses(s: String) -> bool {
        // placing in dummy val to guarentee no failure with last().unwrap()
        let mut stack = vec!['#'];
        let close_match = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);
        for c in s.chars() {
            if close_match.contains_key(&c) {
                if *close_match.get(&c).unwrap() == *stack.last().unwrap() {
                    stack.pop();
                } else {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }
        stack.len() == 1
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let s = "()".to_string();
        let out = true;
        assert_eq!(out, Solution::valid_parentheses(s));

        let s = "()[]{}".to_string();
        let out = true;
        assert_eq!(out, Solution::valid_parentheses(s));

        let s = "(]".to_string();
        let out = false;
        assert_eq!(out, Solution::valid_parentheses(s));

        let s = "]".to_string();
        let out = false;
        assert_eq!(out, Solution::valid_parentheses(s));
    }
}
