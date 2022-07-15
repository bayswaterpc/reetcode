use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn rotate_image(intervals: &mut Vec<Vec<i32>>) {
        solutions::quad_pixel_swap::rotate(intervals)
    }
}

pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;
    use std::fmt::Debug;
    #[allow(dead_code)]
    pub fn test_rotate<T: Sized + PartialEq + Debug, const M: usize, const N: usize>(
        input: [[T; M]; N],
        output: [[T; M]; N],
        rotate: fn(&mut Vec<Vec<T>>),
    ) {
        let mut intervals = array2d_to_vec2d(input);
        rotate(&mut intervals);
        let output = array2d_to_vec2d(output);
        assert_eq!(output, intervals);
    }
}
