pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let (mut idx, n) = (0, intervals.len());
    let mut output: Vec<Vec<i32>> = vec![];
    while idx < n && new_interval[0] > intervals[idx][0] {
        output.push(intervals[idx].clone());
        idx += 1;
    }

    if output.is_empty() || output.last().unwrap()[1] < new_interval[0] {
        output.push(new_interval);
    } else {
        let output_end = output.last().unwrap()[1];
        output.last_mut().unwrap()[1] = new_interval[1].max(output_end);
    }

    while idx < n {
        let (start, end) = (intervals[idx][0], intervals[idx][1]);
        idx += 1;

        if output.last().unwrap()[1] < start {
            output.push(vec![start, end]);
        } else {
            let output_end = output.last().unwrap()[1];
            output.last_mut().unwrap()[1] = output_end.max(end);
        }
    }
    output
}

pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;

    #[test]
    fn unit() {
        let intervals = array2d_to_vec2d([[1, 3], [6, 9]]);
        let new_interval = vec![2, 5];
        let output = array2d_to_vec2d([[1, 5], [6, 9]]);
        assert_eq!(output, super::insert(intervals, new_interval));

        let intervals = array2d_to_vec2d([[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]]);
        let new_interval = vec![4, 8];
        let output = array2d_to_vec2d([[1, 2], [3, 10], [12, 16]]);
        assert_eq!(output, super::insert(intervals, new_interval));

        let intervals = array2d_to_vec2d([[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]]);
        let new_interval = vec![0, 20];
        let output = array2d_to_vec2d([[0, 20]]);
        assert_eq!(output, super::insert(intervals, new_interval));
    }
}
