use super::Solution;

impl Solution {
    /// [[Jump Game](https://leetcode.com/problems/jump-game/)
    /// #dp
    /// #time=O(n)
    /// #space=O(1)
    //pub fn can_jump(nums: Vec<i32>) -> bool {
    pub fn jump_game(nums: Vec<i32>) -> bool {
        let mut last_good_pos = nums.len() - 1;
        for ii in (0..nums.len() - 1).rev() {
            if (nums[ii] as usize) + ii >= last_good_pos {
                last_good_pos = ii
            }
        }
        last_good_pos == 0
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let nums = vec![2, 3, 1, 1, 4];
        let output = true;
        assert_eq!(output, Solution::jump_game(nums));

        let nums = vec![3, 2, 1, 0, 4];
        let output = false;
        assert_eq!(output, Solution::jump_game(nums));
    }
}
