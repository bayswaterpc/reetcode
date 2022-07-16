pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut missing = nums.len() as i32;
    for (ii, num) in nums.into_iter().enumerate() {
        missing ^= (ii as i32) ^ num;
    }
    missing
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::missing_number_268::test::do_unit;

    #[test] //replace allow with test when ready
            // Use VsCode run or debug options or..
            // change name to a unique test name and run `cargo test unq_name` or  run ..
            // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit([3, 0, 1], 2, super::missing_number);
        do_unit([0, 1], 2, super::missing_number);
        do_unit([9, 6, 4, 2, 3, 5, 7, 0, 1], 8, super::missing_number);
    }
}
