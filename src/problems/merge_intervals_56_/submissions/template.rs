#[allow(unused)] //remove when implementing
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    vec![]
}

pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        let intervals = array2d_to_vec2d([[1, 3], [2, 6], [8, 10], [15, 18]]);
        let output = array2d_to_vec2d([[1, 6], [8, 10], [15, 18]]);
        assert_eq!(output, super::merge(intervals));

        let intervals = array2d_to_vec2d([[1, 4], [4, 5]]);
        let output = array2d_to_vec2d([[1, 5]]);
        assert_eq!(output, super::merge(intervals));
    }
}
