use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        solutions::car_fleet(target, position, speed)
    }
}

pub mod test {
    pub fn do_unit<const M: usize, T: Clone + PartialEq + std::fmt::Debug + Sized>(
        target: T,
        position: [T; M],
        speed: [T; M],
        output: T,
        car_fleet: fn(T, Vec<T>, Vec<T>) -> T,
    ) {
        assert_eq!(output, car_fleet(target, position.to_vec(), speed.to_vec()));
    }
}
