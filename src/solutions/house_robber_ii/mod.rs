use super::house_robber::house_robber;
use super::Solution;

impl Solution {
    /// Solution to [House Robber](https://leetcode.com/problems/house-robber-ii/)
    /// #time=O(N)
    /// #space=O(1)
    //pub fn rob(nums: Vec<i32>) -> i32 {
    pub fn house_robber_ii(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }

        let h1 = house_robber(&nums[1..]);
        let h2 = house_robber(&nums[..(nums.len() - 1)]);
        h1.max(h2)
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let n = vec![2, 3, 2];
        let output = 3;
        assert_eq!(output, Solution::house_robber_ii(n));

        let n = vec![1, 2, 3, 1];
        let output = 4;
        assert_eq!(output, Solution::house_robber_ii(n));

        let n = vec![1, 2, 3];
        let output = 3;
        assert_eq!(output, Solution::house_robber_ii(n));
    }
}
