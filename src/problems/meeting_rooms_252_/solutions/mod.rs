pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut prev = 0;
    for ii in 1..intervals.len() {
        if intervals[prev][1] > intervals[ii][0] {
            return false;
        }
        prev = ii;
    }
    true
}

pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;

    #[test] //replace allow with test when ready
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
        let output = true;
        assert_eq!(output, super::can_attend_meetings(intervals));
    }
}
