use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn meeting_rooms_ii(intervals: Vec<Vec<i32>>) -> i32 {
        solutions::min_meeting_rooms(intervals)
    }
}
