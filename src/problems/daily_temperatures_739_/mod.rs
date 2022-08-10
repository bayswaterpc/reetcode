use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn daily_temperatures_739_(temperatures: Vec<i32>) -> Vec<i32> {
        solutions::daily_temperatures(temperatures)
    }
}

pub mod test {
    pub fn do_unit<const M: usize, T: Clone + PartialEq + std::fmt::Debug + Sized>(
        input: [T; M],
        output: [T; M],
        daily_temperatures: fn(Vec<T>) -> Vec<T>,
    ) {
        assert_eq!(output.to_vec(), daily_temperatures(input.to_vec()));
    }
}
