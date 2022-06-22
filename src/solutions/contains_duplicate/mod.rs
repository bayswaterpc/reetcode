use super::Solution;
use std::collections::HashSet;

impl Solution {
    /// Solution to [Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut n_set = HashSet::new();
        for n in nums {
            if n_set.contains(&n) {
                return true;
            }
            n_set.insert(n);
        }
        false
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn contains_duplicate() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(true, Solution::contains_duplicate(nums));

        let nums = vec![1, 2, 3, 4];
        assert_eq!(false, Solution::contains_duplicate(nums));

        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(true, Solution::contains_duplicate(nums));
    }
}
