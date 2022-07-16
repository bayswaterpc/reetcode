#[allow(non_snake_case)] //remove unused when implementing
pub fn hammingWeight(n: u32) -> i32 {
    let mut bits = 0;
    let mut n = n;
    while n != 0 {
        bits += 1;
        n &= n - 1;
    }
    bits
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::number_of_1_bits::test::do_unit;

    #[test] //replace allow with test when ready
            // Use VsCode run or debug options or..
            // change name to a unique test name and run `cargo test unq_name` or  run ..
            // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit("00000000000000000000000000001011", 3, super::hammingWeight);
        do_unit("00000000000000000000000010000000", 1, super::hammingWeight);
        do_unit("11111111111111111111111111111101", 31, super::hammingWeight);
    }
}
