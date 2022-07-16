#[allow(unused)] //remove unused when implementing
pub fn reverse_bits(x: u32) -> u32 {
    1
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::reverse_bits_190_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(
            "00000010100101000001111010011100",
            964176192,
            super::reverse_bits,
        );
    }
}
