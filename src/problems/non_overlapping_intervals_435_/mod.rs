use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn non_overlapping_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        solutions::erase_overlap_intervals(intervals)
    }
}
