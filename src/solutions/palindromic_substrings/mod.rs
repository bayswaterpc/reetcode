use super::Solution;

impl Solution {
    // pub fn count_substrings(s: String) -> i32 {
    pub fn palindromic_substrings(s: String) -> i32 {
        fn get_palindromic_substring_count_around_center(s: &[char], ii: usize, jj: usize) -> i32 {
            let mut ii = ii;
            let mut jj = jj;
            let mut count = 0;
            // usize ii will wrap around to max after zero becoming larger than jj
            while ii <= jj && jj < s.len() && s[ii] == s[jj] {
                count += 1;
                ii = ii.wrapping_sub(1);
                jj = jj.wrapping_add(1);
            }
            count
        }
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        for ii in 0..chars.len() {
            count += get_palindromic_substring_count_around_center(&chars, ii, ii);
            count += get_palindromic_substring_count_around_center(&chars, ii, ii + 1);
        }
        count
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let s = "abc".to_string();
        let output = 3;
        assert_eq!(output, Solution::palindromic_substrings(s));

        let s = "aaa".to_string();
        let output = 6;
        assert_eq!(output, Solution::palindromic_substrings(s));

        let s = "aba".to_string();
        let output = 4;
        assert_eq!(output, Solution::palindromic_substrings(s));
    }
}
