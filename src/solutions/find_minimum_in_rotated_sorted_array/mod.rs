use super::Solution;
use crate::utils::solution_utils::find_pivoted_start_index;

impl Solution {
    /// Solution to [Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/)
    //pub fn find_min(nums: Vec<i32>) -> i32 {
    pub fn find_minimum_in_rotated_sorted_array(nums: Vec<i32>) -> i32 {
        let index = find_pivoted_start_index(nums.as_slice()).unwrap();
        nums[index]
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let nums = vec![1];
        let output = 1;
        assert_eq!(output, Solution::find_minimum_in_rotated_sorted_array(nums));

        let nums = vec![3, 4, 5, 1, 2];
        let output = 1;
        assert_eq!(output, Solution::find_minimum_in_rotated_sorted_array(nums));

        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let output = 0;
        assert_eq!(output, Solution::find_minimum_in_rotated_sorted_array(nums));

        let nums = vec![11, 13, 15, 17];
        let output = 11;
        assert_eq!(output, Solution::find_minimum_in_rotated_sorted_array(nums));
    }
}
