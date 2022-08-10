use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut deque: VecDeque<(i32, i32)> = VecDeque::new();
    let mut res = Vec::new();
    for ii in 0..nums.len() {
        let val = nums[ii];
        let ii = ii as i32;
        while !deque.is_empty() && deque.back().unwrap().0 < val {
            deque.pop_back();
        }

        deque.push_back((val, ii));
        while ii - deque.front().unwrap().1 + 1 > k {
            deque.pop_front();
        }

        if ii >= k - 1 {
            res.push(deque.front().unwrap().0);
        }
    }
    res
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::sliding_window_maximum_239_::test::do_unit;

    #[test]
    fn unit() {
        do_unit([1, -1], 1, [1, -1], super::max_sliding_window);
        do_unit(
            [1, 3, -1, -3, 5, 3, 6, 7],
            3,
            [3, 3, 5, 5, 6, 7],
            super::max_sliding_window,
        );
        do_unit([1], 1, [1], super::max_sliding_window);
    }
}
