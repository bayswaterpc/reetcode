use std::collections::BinaryHeap;

#[derive(Default)]
pub struct MedianFinder {
    hi: BinaryHeap<i32>,
    lo: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            hi: BinaryHeap::default(),
            lo: BinaryHeap::default(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        //hi +1 in size
        self.lo.push(num);
        self.hi.push(-self.lo.peek().unwrap());
        self.lo.pop();

        // lo +1 in size hi -1
        if self.lo.len() < self.hi.len() {
            self.lo.push(-self.hi.peek().unwrap());
            self.hi.pop();
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.lo.is_empty() {
            panic!("Cannot call empty on empty data")
        } else if self.hi.len() < self.lo.len() {
            *self.lo.peek().unwrap() as f64
        } else {
            (*self.lo.peek().unwrap() - *self.hi.peek().unwrap()) as f64 / 2.0
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

pub mod test {
    use super::MedianFinder;
    use crate::utils::test_utils::build_string_vec_from_str_line;

    pub fn test_median_finder(in_commands: &[&str], in_vals: &[i32], out_vals: &str) {
        if in_commands.is_empty()
            || in_vals.len() != in_commands.len()
            || in_commands[0] != "MedianFinder"
        {
            panic!("Invalid input")
        }

        let expected_outputs = build_string_vec_from_str_line(out_vals);
        if expected_outputs.len() != in_vals.len() {
            panic!("Invalid expected output")
        }
        let mut median_finder = MedianFinder::new();
        for (ii, command) in in_commands.iter().skip(1).enumerate() {
            match *command {
                "addNum" => {
                    median_finder.add_num(in_vals[ii + 1]);
                }
                "findMedian" => {
                    let val = median_finder.find_median();
                    let expected: f64 = expected_outputs[ii + 1].parse().expect("should be f64");
                    assert_eq!(val, expected);
                }
                _ => panic!("Invalid input command"),
            }
        }
    }

    #[test]
    pub fn unit() {
        let in_commands = [
            "MedianFinder",
            "addNum",
            "addNum",
            "findMedian",
            "addNum",
            "findMedian",
        ];
        let in_vals = [i32::MIN, 1, 2, i32::MIN, 3, i32::MIN];
        let out_vals = "null, null, null, 1.5, null, 2.0";
        test_median_finder(&in_commands, &in_vals, out_vals);

        let in_commands = [
            "MedianFinder",
            "addNum",
            "findMedian",
            "addNum",
            "findMedian",
            "addNum",
            "findMedian",
            "addNum",
            "findMedian",
            "addNum",
            "findMedian",
            "addNum",
            "findMedian",
            "addNum",
            "findMedian",
            "addNum",
            "findMedian",
            "addNum",
            "findMedian",
            "addNum",
            "findMedian",
            "addNum",
            "findMedian",
        ];
        let in_vals = [
            i32::MIN,
            6,
            i32::MIN,
            10,
            i32::MIN,
            2,
            i32::MIN,
            6,
            i32::MIN,
            5,
            i32::MIN,
            0,
            i32::MIN,
            6,
            i32::MIN,
            3,
            i32::MIN,
            1,
            i32::MIN,
            0,
            i32::MIN,
            0,
            i32::MIN,
        ];
        let out_vals = "null,null,6.00000,null,8.00000,null,6.00000,null,6.00000,null,6.00000,null,5.50000,null,6.00000,null,5.50000,null,5.00000,null,4.00000,null,3.00000";
        test_median_finder(&in_commands, &in_vals, out_vals);
    }
}
