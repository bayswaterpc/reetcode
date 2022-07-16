#[allow(unused)] //remove unused when implementing
pub fn is_palindrome(s: String) -> bool {
    true
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::valid_palindrome_125_::test::do_unit;

    //replace allow with #[test] when ready
    #[allow(dead_code)]
    //#[test]
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit("a", true, super::is_palindrome);
        do_unit("A man, a plan, a canal: Panama", true, super::is_palindrome);
        do_unit("race a car", false, super::is_palindrome);
        do_unit(" ", true, super::is_palindrome);
    }
}
