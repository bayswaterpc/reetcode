pub mod solutions;
pub mod submissions;

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
    use std::fmt::Debug;

    #[allow(dead_code)]
    pub fn do_unit<const N: usize, T: Clone + Debug + PartialEq, U: PartialEq + Debug>(
        input: [T; N],
        output: U,
        ff: fn(Vec<T>) -> U,
    ) {
        assert_eq!(output, ff(input.to_vec()));
    }
}
