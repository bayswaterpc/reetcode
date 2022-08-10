pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut output = vec![intervals[0].clone()];
    for int in intervals.into_iter().skip(1) {
        if output.last().unwrap()[1] < int[0] {
            output.push(int);
        } else {
            let last_output = output.last().unwrap()[1];
            output.last_mut().unwrap()[1] = last_output.max(int[1]);
        }
    }

    output
}

pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;

    #[test]
    fn unit() {
        let intervals = array2d_to_vec2d([[1, 3], [2, 6], [8, 10], [15, 18]]);
        let output = array2d_to_vec2d([[1, 6], [8, 10], [15, 18]]);
        assert_eq!(output, super::merge(intervals));

        let intervals = array2d_to_vec2d([[1, 4], [4, 5]]);
        let output = array2d_to_vec2d([[1, 5]]);
        assert_eq!(output, super::merge(intervals));
    }
}
