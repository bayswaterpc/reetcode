use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn merge_interval(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        solutions::merge(intervals)
    }
}
