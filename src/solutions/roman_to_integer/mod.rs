use super::Solution;
use std::collections::HashMap;

impl Solution {
    /// Solution to [Roman to Integer](https://leetcode.com/problems/roman-to-integer/)
    pub fn roman_to_int(s: String) -> i32 {
        let roman_numerals: HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();

        let mut res = 0_i32;
        let mut prev = -1_i32;
        for c in s.chars().rev() {
            let val = roman_numerals.get(&c).unwrap();
            if *val < prev {
                res -= val;
            } else {
                prev = *val;
                res += *val;
            }
        }
        res
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn roman_to_int() {
        let i = Solution::roman_to_int("LVIII".to_string());
        assert_eq!(58, i);
    }
}
