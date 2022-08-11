use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn koko_eating_bananas_875_(piles: Vec<i32>, h: i32) -> i32 {
        solutions::min_eating_speed(piles, h)
    }
}

pub mod test {
    pub fn do_unit<const M: usize, T: Clone + PartialEq + std::fmt::Debug + Sized>(
        piles: [T; M],
        h: T,
        output: T,
        min_eating_speed: fn(Vec<T>, T) -> T,
    ) {
        assert_eq!(output, min_eating_speed(piles.to_vec(), h));
    }
}
