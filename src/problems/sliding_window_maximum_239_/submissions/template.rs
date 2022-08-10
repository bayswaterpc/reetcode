#[allow(unused)] //remove unused when implementing
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    vec![]
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::sliding_window_maximum_239_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(
            [1, 3, -1, -3, 5, 3, 6, 7],
            3,
            [3, 3, 5, 5, 6, 7],
            super::max_sliding_window,
        );
        do_unit([1], 1, [1], super::max_sliding_window);
    }
}
