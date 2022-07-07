use super::Solution;

pub fn maximum_subarray(nums: &[i32]) -> i32 {
    let (mut curr_sum, mut max_sum) = (nums[0], nums[0]);
    for n in nums.iter().skip(1) {
        curr_sum = (*n).max(curr_sum + *n);
        max_sum = max_sum.max(curr_sum)
    }
    max_sum
}

impl Solution {
    /// Solution to [Maximum Subarray](https://leetcode.com/problems/maximum-subarray/)
    /// Kadane's algorithm
    /// #space=O(1)
    /// #time=O(N)
    //pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    pub fn maximum_subarray(nums: Vec<i32>) -> i32 {
        maximum_subarray(&nums)
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let output = 6;
        assert_eq!(output, Solution::maximum_subarray(nums));

        let nums = vec![1];
        let output = 1;
        assert_eq!(output, Solution::maximum_subarray(nums));

        let nums = vec![5, 4, -1, 7, 8];
        let output = 23;
        assert_eq!(output, Solution::maximum_subarray(nums));
    }
}
