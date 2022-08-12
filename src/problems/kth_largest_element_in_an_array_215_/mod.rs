use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        solutions::find_kth_largest(nums, k)
    }
}

pub mod test {
    pub fn do_unit<const N: usize, T: Clone + PartialEq + std::fmt::Debug + Sized>(
        nums: [T; N],
        k: T,
        output: T,
        f: fn(Vec<T>, T) -> T,
    ) {
        assert_eq!(output, f(nums.to_vec(), k));
    }
}
