#[allow(unused)] //remove unused when implementing
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    vec![]
}
pub mod test {
    #[allow(unused_imports)]
    use crate::problems::generate_parentheses_22_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit( 3, ["((()))","(()())","(())()","()(())","()()()"], super::generate_parenthesis);
        do_unit( 1, ["()"], super::generate_parenthesis);
    }
}
