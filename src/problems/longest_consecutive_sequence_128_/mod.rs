use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn longest_consecutive_sequence_128_(nums: Vec<i32>) -> i32 {
        solutions::longest_consecutive(nums)
    }
}

pub mod test {
    #[allow(dead_code)]
    pub fn do_unit<const N: usize, T: Clone + std::fmt::Debug + PartialEq>(
        input: [T; N],
        output: T,
        longest_consecutive: fn(Vec<T>) -> T,
    ) {
        assert_eq!(output, longest_consecutive(input.to_vec()));
    }
}
