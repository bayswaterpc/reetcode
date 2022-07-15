use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn meeting_rooms(intervals: Vec<Vec<i32>>) -> bool {
        solutions::can_attend_meetings(intervals)
    }
}
