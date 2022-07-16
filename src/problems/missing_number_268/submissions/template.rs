#[allow(unused)] //remove unused when implementing
pub fn missing_number(nums: Vec<i32>) -> i32 {
    1
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::missing_number_268::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit([3, 0, 1], 2, super::missing_number);
    }
}
