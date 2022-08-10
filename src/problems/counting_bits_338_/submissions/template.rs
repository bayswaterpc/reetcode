#[allow(unused)] //remove unused when implementing
pub fn count_bits(n: i32) -> Vec<i32> {
    vec![]
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::counting_bits_338_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(2, [0, 1, 1], super::count_bits);
        do_unit(5, [0, 1, 1, 2, 1, 2], super::count_bits);
    }
}
