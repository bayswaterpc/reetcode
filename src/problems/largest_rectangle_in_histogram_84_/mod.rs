use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    //lol there is disagreement in spelling between leetcodes function call and problem slug
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        solutions::largest_rectangle_area(heights)
    }
}

pub mod test {
    pub fn do_unit<const M: usize, T: Clone + PartialEq + std::fmt::Debug + Sized>(
        input: [T; M],
        output: T,
        f: fn(Vec<T>) -> T,
    ) {
        assert_eq!(output, f(input.to_vec()));
    }
}
