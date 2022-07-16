#[allow(unused)] //remove unused when implementing
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    1
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::longest_consecutive_sequence_128_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit([100, 4, 200, 1, 3, 2], 4, super::longest_consecutive);
        do_unit(
            [0, 3, 7, 2, 5, 8, 4, 6, 0, 1],
            9,
            super::longest_consecutive,
        );
    }
}
