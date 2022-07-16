use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn counting_bits(n: i32) -> Vec<i32> {
        solutions::count_bits(n)
    }
}

pub mod test {
    #[allow(dead_code)]
    pub fn do_unit<const M: usize, T: Sized + PartialEq + std::fmt::Debug + Clone>(
        input: T,
        output: [T; M],
        count_bits: fn(T) -> Vec<T>,
    ) {
        assert_eq!(output.to_vec(), count_bits(input));
    }
}
