use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn max_area_of_island_695_(grid: Vec<Vec<i32>>) -> i32 {
        solutions::max_area_of_island(grid)
    }
}

pub mod test {
    use crate::utils::test_utils::array2d_to_vec2d;
    pub fn do_unit<
        const N: usize,
        const M: usize,
        T: Clone + PartialEq + std::fmt::Debug + Sized,
    >(
        grid: [[T; N]; M],
        output: T,
        f: fn(Vec<Vec<T>>) -> T,
    ) {
        assert_eq!(output, f(array2d_to_vec2d(grid)));
    }
}
