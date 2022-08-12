use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn task_scheduler_621_(nums: Vec<char>, n: i32) -> i32 {
        solutions::least_interval(nums, n)
    }
}

pub mod test {
    pub fn do_unit<
        const N: usize,
        T: Clone + PartialEq + std::fmt::Debug + Sized,
        R: Clone + PartialEq + std::fmt::Debug + Sized,
    >(
        nums: [T; N],
        n: R,
        output: R,
        f: fn(Vec<T>, R) -> R,
    ) {
        assert_eq!(output, f(nums.to_vec(), n));
    }
}
