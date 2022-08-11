#[allow(unused)] //remove unused when implementing
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    0
}

pub mod test {
    #[allow(unused_imports)]
    use super::super::super::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit([2, 1, 5, 6, 2, 3], 10, super::largest_rectangle_area);
        do_unit([2, 4], 4, super::largest_rectangle_area);
    }
}
