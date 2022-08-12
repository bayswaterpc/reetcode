use std::cmp::Reverse;
use std::collections::BinaryHeap;
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut minheap = BinaryHeap::with_capacity(k);

    nums.into_iter().for_each(|i| minheap.push(Reverse(i)));
    while minheap.len() > k as usize {
        minheap.pop();
    }
    minheap.peek().unwrap().0
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::kth_largest_element_in_an_array_215_::test::do_unit;

    #[test]
    fn unit() {
        do_unit([3, 2, 1, 5, 6, 4], 2, 5, super::find_kth_largest);
        do_unit([3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4, super::find_kth_largest);
    }
}
