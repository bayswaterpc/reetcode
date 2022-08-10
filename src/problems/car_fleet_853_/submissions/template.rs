#[allow(unused)] //remove unused when implementing
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    0
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::car_fleet_853_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(12, [10, 8, 0, 5, 3], [2, 4, 1, 1, 3], 3, super::car_fleet);
        do_unit(10, [3], [3], 1, super::car_fleet);
        do_unit(100, [0, 2, 4], [4, 2, 1], 1, super::car_fleet);
    }
}
