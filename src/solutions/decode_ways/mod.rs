use super::Solution;

impl Solution {
    /// Solution to [Decode Ways](https://leetcode.com/problems/decode-ways/)
    /// #time=O(N)
    /// #space=O(1)
    //pub fn num_decodings(s: String) -> i32 {
    pub fn decode_ways(s: String) -> i32 {
        fn is_valid_code(code: u8) -> bool {
            (1..=26).contains(&code)
        }
        let mut prev_prev = 1;
        let mut prev = 1;
        let mut prev_digit = 0;

        // because we deal with digits (0-9) they are encoded by one byte
        // - so it's ok to use bytes iterator
        for digit in s.bytes().map(|b| b - b'0') {
            let mut num = 0;

            if is_valid_code(digit) {
                num += prev;
            }

            if prev_digit != 0 && is_valid_code(prev_digit * 10 + digit) {
                num += prev_prev;
            }

            if num == 0 {
                return 0;
            }

            prev_prev = prev;
            prev = num;
            prev_digit = digit;
        }
        prev
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let s = "12".to_string();
        let output = 2;
        assert_eq!(output, Solution::decode_ways(s));

        let s = "226".to_string();
        let output = 3;
        assert_eq!(output, Solution::decode_ways(s));

        let s = "605".to_string();
        let output = 0;
        assert_eq!(output, Solution::decode_ways(s));

        let s = "10".to_string();
        let output = 1;
        assert_eq!(output, Solution::decode_ways(s));
    }
}
