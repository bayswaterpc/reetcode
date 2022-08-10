use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn trapping_rain_water_42_(height: Vec<i32>) -> i32 {
        solutions::trap(height)
    }
}

pub mod test {
    #[allow(dead_code)]
    pub fn do_unit<T: Clone + std::fmt::Debug + PartialEq, const M: usize>(
        height: [T; M],
        output: T,
        trap: fn(Vec<T>) -> T,
    ) {
        assert_eq!(output, trap(height.to_vec()));
    }
}
