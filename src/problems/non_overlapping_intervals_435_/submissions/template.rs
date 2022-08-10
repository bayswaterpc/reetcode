#[allow(unused)] //remove when implementing
pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    0
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
        let intervals = array2d_to_vec2d([[1, 2], [2, 3], [3, 4], [1, 3]]);
        let output = 1;
        assert_eq!(output, super::erase_overlap_intervals(intervals));

        let intervals = array2d_to_vec2d([[1, 2], [1, 2], [1, 2]]);
        let output = 2;
        assert_eq!(output, super::erase_overlap_intervals(intervals));

        let intervals = array2d_to_vec2d([[1, 2], [2, 3]]);
        let output = 0;
        assert_eq!(output, super::erase_overlap_intervals(intervals));
    }
}
