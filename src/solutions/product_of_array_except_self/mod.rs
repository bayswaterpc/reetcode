use super::Solution;

impl Solution {
    /// Solution to [Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/)
    pub fn product_of_array_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut lhps = vec![1; nums.len()];
        let mut rhps = vec![1; nums.len()];
        let mut ans = vec![0; nums.len()];

        for ii in 1..nums.len() {
            lhps[ii] = lhps[ii - 1] * nums[ii - 1];
        }

        for ii in (0..(nums.len() - 1)).rev() {
            rhps[ii] = rhps[ii + 1] * nums[ii + 1];
        }

        for ii in 0..nums.len() {
            ans[ii] = lhps[ii] * rhps[ii];
        }

        ans
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let nums = vec![1, 2, 3, 4];
        let output = vec![24, 12, 8, 6];
        assert_eq!(output, Solution::product_of_array_except_self(nums));

        let nums = vec![-1, 1, 0, -3, 3];
        let output = vec![0, 0, 9, 0, 0];
        assert_eq!(output, Solution::product_of_array_except_self(nums));
    }
}
