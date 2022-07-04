use super::Solution;

pub fn house_robber(nums: &[i32]) -> i32 {
    let mut t1 = 0;
    let mut t2 = 0;
    for current in nums.iter() {
        let temp = t1;
        t1 = t1.max(current + t2);
        t2 = temp;
    }
    t1
}

impl Solution {
    /// Solution to [House Robber](https://leetcode.com/problems/house-robber/)
    /// #time=O(N)
    /// #space=O(1)
    //pub fn rob(nums: Vec<i32>) -> i32 {
    pub fn house_robber(nums: Vec<i32>) -> i32 {
        house_robber(&nums)
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let n = vec![1, 2, 3, 1];
        let output = 4;
        assert_eq!(output, Solution::house_robber(n));

        let n = vec![2, 7, 9, 3, 1];
        let output = 12;
        assert_eq!(output, Solution::house_robber(n));

        let n = vec![0];
        let output = 0;
        assert_eq!(output, Solution::house_robber(n));
    }
}
