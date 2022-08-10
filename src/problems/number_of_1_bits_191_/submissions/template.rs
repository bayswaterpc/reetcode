#[allow(non_snake_case, unused)] //remove unused when implementing
pub fn hammingWeight(n: u32) -> i32 {
    1
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::number_of_1_bits_191_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit("00000000000000000000000000001011", 3, super::hammingWeight);
        do_unit("00000000000000000000000010000000", 1, super::hammingWeight);
        do_unit("11111111111111111111111111111101", 31, super::hammingWeight);
    }
}
