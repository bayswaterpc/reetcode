use super::Solution;

/// Solution to [longest_substring_without_repeating_characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/)
//pub fn max_area(height: Vec<i32>) -> i32 {
impl Solution {
    pub fn longest_substring_without_repeating_characters(s: String) -> i32 {
        let mut last_index_without_char_map = [0; 127];
        let mut ll = 0;
        let mut ret = 0;

        for (right, ch) in s.char_indices().map(|(i, c)| (i as i32, c as usize)) {
            ll = ll.max(last_index_without_char_map[ch]);
            ret = ret.max(right + 1 - ll);
            last_index_without_char_map[ch] = right + 1;
        }

        ret
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let height = "abcabcbb".to_string();
        let output = 3;
        assert_eq!(
            output,
            Solution::longest_substring_without_repeating_characters(height)
        );

        let height = "bbbbb".to_string();
        let output = 1;
        assert_eq!(
            output,
            Solution::longest_substring_without_repeating_characters(height)
        );

        let height = "pwwkew".to_string();
        let output = 3;
        assert_eq!(
            output,
            Solution::longest_substring_without_repeating_characters(height)
        );
    }
}
