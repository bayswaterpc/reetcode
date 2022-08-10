#[allow(unused)] //remove unused when implementing
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    vec![]
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::daily_temperatures_739_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(
            [73, 74, 75, 71, 69, 72, 76, 73],
            [1, 1, 4, 2, 1, 1, 0, 0],
            super::daily_temperatures,
        );
        do_unit([30, 40, 50, 60], [1, 1, 1, 0], super::daily_temperatures);
        do_unit([30, 60, 90], [1, 1, 0], super::daily_temperatures);
    }
}
