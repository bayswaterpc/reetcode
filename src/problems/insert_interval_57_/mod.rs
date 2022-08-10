use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn insert_interval(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        solutions::insert(intervals, new_interval)
    }
}
