use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn missing_number_268(nums: Vec<i32>) -> i32 {
        solutions::missing_number(nums)
    }
}

pub mod test {
    #[allow(dead_code)]
    pub fn do_unit<const N: usize, T: Clone + std::fmt::Debug + PartialEq>(
        input: [T; N],
        output: T,
        missing_number: fn(Vec<T>) -> T,
    ) {
        let input = input.to_vec();
        assert_eq!(output, missing_number(input));
    }
}
