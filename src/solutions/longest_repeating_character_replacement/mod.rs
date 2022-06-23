use super::Solution;
//#time=O(n)
//#space=O(1)

/// Solution to [Longest Repeating Character Replacement](https://leetcode.com/problems/longest-repeating-character-replacement/)
impl Solution {
    // Use below in leetcode
    //pub fn character_replacement(s: String, k: i32) -> i32 {
    pub fn longest_repeating_character_replacement(s: String, k: i32) -> i32 {
        let len_s: usize = s.len();
        let chs_s: Vec<char> = s.chars().collect();
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        let mut freqs: Vec<u16> = vec![0; 26];
        let mut longest: usize = 0;
        let mut most: u16 = 0;

        while hi < len_s {
            let freq_hi = &mut freqs[chs_s[hi] as usize - 'A' as usize];
            *freq_hi += 1;
            most = std::cmp::max(most, *freq_hi);
            while hi as u16 - lo as u16 + 1 - most > k as u16 {
                freqs[chs_s[lo] as usize - 'A' as usize] -= 1;
                lo += 1;
            }
            longest = std::cmp::max(longest, hi - lo + 1);
            hi += 1;
        }
        longest as i32
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let s = "ABAB".to_string();
        let k = 2;
        let output = 4;
        assert_eq!(
            output,
            Solution::longest_repeating_character_replacement(s, k)
        );

        let s = "AABABBA".to_string();
        let k = 2;
        let output = 5;
        assert_eq!(
            output,
            Solution::longest_repeating_character_replacement(s, k)
        );
    }
}
