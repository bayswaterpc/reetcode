use std::collections::HashSet;

/// `#array&hashset`
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let nums_len = nums.len();
    nums_len != nums.into_iter().collect::<HashSet<i32>>().len()
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::contains_duplicate_217_::test::do_unit;

    #[test] //replace allow with test when ready
            // Use VsCode run or debug options or..
            // change name to a unique test name and run `cargo test unq_name` or  run ..
            // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit([1, 2, 3, 1], true, super::contains_duplicate);
        do_unit([1, 2, 3, 4], false, super::contains_duplicate);
        do_unit(
            [1, 1, 1, 3, 3, 4, 3, 2, 4, 2],
            true,
            super::contains_duplicate,
        );
    }
}
