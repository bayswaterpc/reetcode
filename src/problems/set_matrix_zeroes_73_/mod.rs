use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn set_matrix_zeroes(matrix: &mut Vec<Vec<i32>>) {
        solutions::set_zeroes(matrix)
    }
}

pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;
    use std::fmt::Debug;
    #[allow(dead_code)]
    pub fn do_unit<T: Sized + PartialEq + Debug + Clone, const M: usize, const N: usize>(
        input: [[T; M]; N],
        output: [[T; M]; N],
        set_zeroes: fn(&mut Vec<Vec<T>>),
    ) {
        let mut input = array2d_to_vec2d(input);
        let output = array2d_to_vec2d(output);
        set_zeroes(&mut input);
        assert_eq!(output, input);
    }
}
