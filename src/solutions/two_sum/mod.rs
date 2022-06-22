use super::Solution;
use std::collections::HashMap;

/// [Two Sum](https://leetcode.com/problems/two-sum/solution/)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut rem_to_pos = HashMap::new();
        let mut ii = 0_i32;
        for nn in nums {
            let rem = target - nn;
            if rem_to_pos.contains_key(&rem) {
                return vec![*rem_to_pos.get(&rem).unwrap(), ii];
            }
            rem_to_pos.insert(nn, ii);
            ii += 1;
        }
        vec![-1, -1]
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(vec![0, 1], Solution::two_sum(nums, target));
        // Because nums[0] + nums[1] == 9, we return [0, 1].

        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(vec![1, 2], Solution::two_sum(nums, target));

        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(vec![0, 1], Solution::two_sum(nums, target));
    }
}
