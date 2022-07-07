use super::Solution;

pub fn maximum_product_subarray(nums: &[i32]) -> i32 {
    let mut min_chain = nums[0];
    let mut max_chain = nums[0];
    let mut res = nums[0];

    for n in nums.iter().skip(1) {
        let temp_max = (*n).max((max_chain * (*n)).max(min_chain * (*n)));
        min_chain = (*n).min((max_chain * (*n)).min(min_chain * (*n)));
        max_chain = temp_max;

        res = res.max(max_chain)
    }

    res
}

impl Solution {
    /// Solution to [Maximum Subarray](https://leetcode.com/problems/maximum-subarray/)
    /// a modified kadane's algorithm
    /// #space=O(1)
    /// #time=O(N)
    //pub fn max_product(nums: Vec<i32>) -> i32 {
    pub fn maximum_product_subarray(nums: Vec<i32>) -> i32 {
        maximum_product_subarray(&nums)
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let nums = vec![2, 3, -2, 4];
        let output = 6;
        assert_eq!(output, Solution::maximum_product_subarray(nums));

        let nums = vec![-2, 0, -1];
        let output = 0;
        assert_eq!(output, Solution::maximum_product_subarray(nums));
    }
}
