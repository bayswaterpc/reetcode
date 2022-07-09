use super::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    /// Solution to vec![Clone Graph](https://leetcode.com/problems/course-schedule/)
    /// #time=O(E+V) where E is the number of courses and V is the number of prerequisites
    /// #space=O(E+V)
    pub fn course_schedule(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        solutions::topological_sort::can_finish(num_courses, prerequisites)
        //solutions::backtracking::can_finish(num_courses, prerequisites)
    }
}
