#[allow(unused)] //remove when implementing
pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
    1
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
        let output = 2;
        assert_eq!(output, super::min_meeting_rooms(intervals));

        let intervals = array2d_to_vec2d([[7, 10], [2, 4]]);
        let output = 1;
        assert_eq!(output, super::min_meeting_rooms(intervals));

        let intervals = array2d_to_vec2d([[2, 15], [36, 45], [9, 29], [16, 23], [4, 9]]);
        let output = 2;
        assert_eq!(output, super::min_meeting_rooms(intervals));
    }
}
