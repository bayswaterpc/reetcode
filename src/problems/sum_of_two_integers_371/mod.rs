use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn sum_of_two_integers_371(a: i32, b: i32) -> i32 {
        solutions::get_sum(a, b)
    }
}

pub mod test {
    #[allow(dead_code)]
    pub fn do_unit<T: Clone + std::fmt::Debug + PartialEq>(
        input: (T, T),
        output: T,
        get_sum: fn(T, T) -> T,
    ) {
        assert_eq!(output, get_sum(input.0, input.1));
    }
}
