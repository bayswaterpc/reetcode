use crate::problems::encode_and_decode_strings_271_::EncodeDecode;

#[allow(unused)] //remove unused when implementing
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl EncodeDecode for Codec {
    fn new() -> Self {
        Codec {}
    }

    #[allow(unused)] //remove unused when implementing
    fn encode(&self, strs: Vec<String>) -> String {
        "".to_string()
    }

    #[allow(unused)] //remove unused when implementing
    fn decode(&self, s: String) -> Vec<String> {
        vec![]
    }
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::encode_and_decode_strings_271_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit::<2, super::Codec>(["Hello", "World"]);
        do_unit::<1, super::Codec>([""]);
    }
}
