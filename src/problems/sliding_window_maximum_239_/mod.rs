use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        solutions::max_sliding_window(nums, k)
    }
}

pub mod test {
    #[allow(dead_code)]
    pub fn do_unit<const M: usize, const K: usize, T: Sized + PartialEq + std::fmt::Debug + Clone>(
        nums: [T; M], 
        k: T,
        output: [T; K],
        max_sliding_window: fn(nums: Vec<T>, k: T) -> Vec<T>,
    ) {
        assert_eq!(output.to_vec(), max_sliding_window(nums.to_vec(), k));
    }
}
