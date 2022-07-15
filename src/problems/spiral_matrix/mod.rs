use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        solutions::spiral_order(matrix)
    }
}

pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;
    use std::fmt::Debug;
    #[allow(dead_code)]
    pub fn do_unit<
        T: Sized + PartialEq + Debug + Clone,
        const M: usize,
        const N: usize,
        const Z: usize,
    >(
        input: [[T; M]; N],
        // output: [T; M*N], would be ideal but not supported
        output: [T; Z],
        spiral_order: fn(Vec<Vec<T>>) -> Vec<T>,
    ) {
        let input = array2d_to_vec2d(input);
        assert_eq!(output.to_vec(), spiral_order(input));
    }
}
