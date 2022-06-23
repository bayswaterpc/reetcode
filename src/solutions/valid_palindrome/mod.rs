use super::Solution;
//#category=twopointer
//#space=O(N)
//#space=1

/// [Valid Palindrome](https://leetcode.com/problems/valid-palindrome/)
impl Solution {
    //Below required for leetcode
    //pub fn is_palindrome(s: String) -> bool {
    pub fn valid_palindrome(s: String) -> bool {
        let s_u8 = s.as_bytes();
        let mut ll = 0_i32;
        let mut rr = (s_u8.len() - 1) as i32;

        while ll < rr {
            while ll < rr && !(s_u8[ll as usize] as char).is_alphanumeric() {
                ll += 1;
            }
            while ll < rr && !(s_u8[rr as usize] as char).is_alphanumeric() {
                rr -= 1;
            }

            if (s_u8[rr as usize] as char).to_ascii_lowercase()
                != (s_u8[ll as usize] as char).to_ascii_lowercase()
            {
                return false;
            }
            ll += 1;
            rr -= 1;
        }

        true
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let s = "a.".to_string();
        let out = true;
        assert_eq!(out, Solution::valid_palindrome(s));

        let s = "A man, a plan, a canal: Panama".to_string();
        let out = true;
        assert_eq!(out, Solution::valid_palindrome(s));

        let s = "race a car".to_string();
        let out = false;
        assert_eq!(out, Solution::valid_palindrome(s));

        let s = " ".to_string();
        let out = true;
        assert_eq!(out, Solution::valid_palindrome(s));
    }
}
