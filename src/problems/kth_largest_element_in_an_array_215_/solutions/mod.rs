pub mod heap;
pub mod quicksort;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    quicksort::find_kth_largest(nums, k)
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
