use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn two_sum_ii_167_(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        solutions::two_sum(numbers, target)
    }
}

pub mod test {
    #[allow(dead_code)]
    pub fn do_unit<T: Clone + std::fmt::Debug + PartialEq, const M: usize>(
        numbers: [T; M],
        target: T,
        output: [T; 2],
        two_sum: fn(Vec<T>, T) -> Vec<T>,
    ) {
        assert_eq!(output.to_vec(), two_sum(numbers.to_vec(), target));
    }
}
