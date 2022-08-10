#[allow(unused)] //remove unused when implementing
pub fn trap(height: Vec<i32>) -> i32 {
    0
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::trapping_rain_water_42_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6, super::trap);
        do_unit([4, 2, 0, 3, 2, 5], 9, super::trap);
    }
}
