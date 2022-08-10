use super::Solution;

impl Solution {
    /// Solution to [Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/)
    /// #sliding-window
    /// time=O(N^2)
    /// space=O(1) where m is size of charset
    // in leetcode use below def
    //pub fn longest_palindrome(s: String) -> String {
    pub fn longest_palindromic_substring(s: String) -> String {
        fn expand_around_center(s: &[char], ii: usize, jj: usize) -> (usize, usize) {
            let mut ii = ii;
            let mut jj = jj;

            while ii <= jj && jj < s.len() && s[ii] == s[jj] {
                ii = ii.wrapping_sub(1);
                jj += 1;
            }

            // Only occurs on overflow so just set left to zero
            if ii > jj {
                return (0, jj);
            }

            (ii + 1, jj)
        }

        let char_vec: Vec<char> = s.chars().collect();
        let mut max_len = 0;
        let mut max_indices = (0, 0);
        for ii in 0..char_vec.len() {
            let p1 = expand_around_center(&char_vec, ii, ii);
            let p2 = expand_around_center(&char_vec, ii, ii + 1);
            let mut max_ii_palin_indices = p1;
            if p2.1 - p2.0 > max_ii_palin_indices.1 - max_ii_palin_indices.0 {
                max_ii_palin_indices = p2;
            }
            let pair_len = max_ii_palin_indices.1 - max_ii_palin_indices.0 + 1;
            if pair_len > max_len {
                max_len = pair_len;
                max_indices = max_ii_palin_indices;
            }
        }
        s[max_indices.0..max_indices.1].to_string()
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let s = "babad".to_string();
        let output = "bab".to_string();
        assert_eq!(output, Solution::longest_palindromic_substring(s));

        let s = "cbbd".to_string();
        let output = "bb".to_string();
        assert_eq!(output, Solution::longest_palindromic_substring(s));
    }
}
