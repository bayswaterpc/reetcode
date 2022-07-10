use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        solutions::dp_space_optimized::longest_common_subsequence(text1, text2)
    }
}
