use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn search_a_2d_matrix_74_(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        solutions::search_matrix(matrix, target)
    }
}

pub mod test {
    use crate::utils::test_utils::array2d_to_vec2d;
    pub fn do_unit<
        const M: usize,
        const N: usize,
        T: Clone + PartialEq + std::fmt::Debug + Sized,
        R: Clone + PartialEq + std::fmt::Debug + Sized,
    >(
        matrix: [[T; M]; N],
        target: T,
        output: R,
        search_matrix: fn(Vec<Vec<T>>, T) -> R,
    ) {
        assert_eq!(output, search_matrix(array2d_to_vec2d(matrix), target));
    }
}
