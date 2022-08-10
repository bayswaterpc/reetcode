#[allow(unused)] //remove unused when implementing
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    vec![]
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::two_sum_ii_167_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit([2,7,11,15], 9, [1,2], super::two_sum);
        do_unit([2,3,4], 6, [1,3], super::two_sum);
        do_unit([-1,0], -1, [1,2], super::two_sum);
    }
}
