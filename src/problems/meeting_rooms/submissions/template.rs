#[allow(unused)] //remove when implementing
pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    true
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
        let intervals = array2d_to_vec2d([[0, 30], [5, 10], [15, 20]]);
        let output = false;
        assert_eq!(output, super::can_attend_meetings(intervals));

        let intervals = array2d_to_vec2d([[7, 10], [2, 4]]);
        let output = true;
        assert_eq!(output, super::can_attend_meetings(intervals));

        let intervals = array2d_to_vec2d([[0, 30], [60, 240], [90, 120]]);
        let output = false;
        assert_eq!(output, super::can_attend_meetings(intervals));

        let intervals = array2d_to_vec2d([[13, 15], [1, 13]]);
        let output = false;
        assert_eq!(output, super::can_attend_meetings(intervals));
    }
}
