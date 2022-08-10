#[allow(unused)] //remove unused when implementing
pub fn check_inclusion(s1: String, s2: String) -> bool {
    true
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::permutation_in_string_567_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit("ab", "eidbaooo", true, super::check_inclusion);
        do_unit("ab", "eidboaoo", false, super::check_inclusion);
    }
}
