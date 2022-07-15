pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut prev = 0_usize;
    let mut num_removed = 0;
    for ii in 1..intervals.len() {
        if intervals[ii][0] < intervals[prev][1] {
            if intervals[ii][1] < intervals[prev][1] {
                prev = ii;
            }
            num_removed += 1;
        } else {
            prev = ii;
        }
    }

    num_removed
}

pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;

    #[test] //replace allow with test when ready
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
