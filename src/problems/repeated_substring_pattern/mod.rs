use super::Solution;

impl Solution {
    /// Solution to [Repeated Substring Pattern](https://leetcode.com/problems/repeated-substring-pattern/)
    /// #space=O(1)
    /// #time=O(N*log(N`))
    pub fn repeated_substring_pattern(s: String) -> bool {
        if s.len() == 1 {
            return false;
        }

        let mut num_div = 1_usize;
        while num_div < s.len() {
            num_div += 1;
            if s.len() % num_div != 0 {
                continue;
            }

            let div_size = s.len() / num_div;
            let mut div_start = div_size;
            let mut div_end = div_size * 2;
            let mut all_match = true;
            while div_end <= s.len() {
                if s[0..div_size] != s[div_start..div_end] {
                    all_match = false;
                    break;
                }
                div_start += div_size;
                div_end += div_size;
            }
            if all_match {
                return true;
            }
        }

        false
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::make_linked_list;

    pub fn test_repeated_substring_pattern(input: &str, output: bool) {
        println!("{}", input);
        assert_eq!(
            output,
            Solution::repeated_substring_pattern(input.to_string())
        );
    }

    #[test]
    fn unit() {
        let input = "abcabcabcabc";
        let output = true;
        test_repeated_substring_pattern(input, output);

        let input = "aba";
        let output = false;
        test_repeated_substring_pattern(input, output);

        let input = "babbabbabbabbab";
        let output = true;
        test_repeated_substring_pattern(input, output);
    }
}
