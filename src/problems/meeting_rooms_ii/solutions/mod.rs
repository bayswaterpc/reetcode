use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    // This orders the number of meeting rooms checking start times against end times
    // It will grow as number concurrent meetins grows
    let mut end_time_min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    end_time_min_heap.push(Reverse(intervals[0][1]));

    for interval in intervals.into_iter().skip(1) {
        if end_time_min_heap.peek().unwrap().0 <= interval[0] {
            end_time_min_heap.pop();
        }
        end_time_min_heap.push(Reverse(interval[1]));
    }
    end_time_min_heap.len() as i32
}

pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;

    #[test] //replace allow with test when ready
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
