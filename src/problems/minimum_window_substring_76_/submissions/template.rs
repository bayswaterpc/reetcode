#[allow(unused)] //remove unused when implementing
pub fn min_window(s: String, t: String) -> String {
    "".to_string()
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::minimum_window_substring_76_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(("ADOBECODEBANC", "ABC"), "BANC", super::min_window);
    }
}
