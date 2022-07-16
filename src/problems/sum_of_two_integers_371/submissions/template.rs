#[allow(unused)] //remove unused when implementing
pub fn get_sum(a: i32, b: i32) -> i32 {
    1
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::sum_of_two_integers_371::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit((1, 2), 3, super::get_sum);
        do_unit((2, 3), 5, super::get_sum);
        do_unit((-1, 4), 3, super::get_sum);
    }
}
