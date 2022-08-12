#[allow(unused)] //remove unused when implementing
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    0
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::kth_largest_element_in_an_array_215_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit([3, 2, 1, 5, 6, 4], 2, 5, super::find_kth_largest);
        do_unit([3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4, super::find_kth_largest);
    }
}
