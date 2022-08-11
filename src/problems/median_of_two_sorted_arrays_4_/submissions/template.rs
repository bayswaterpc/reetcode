#[allow(unused)] //remove unused when implementing
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    0.0
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::median_of_two_sorted_arrays_4_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit([1, 2], [2], 2.0, super::find_median_sorted_arrays);
        do_unit([1, 2], [3, 4], 2.5, super::find_median_sorted_arrays);
    }
}
