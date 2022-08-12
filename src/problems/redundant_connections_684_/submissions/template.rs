#[allow(unused)] //remove unused when implementing
pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    vec![]
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::redundant_connections_684_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(
            [[1, 2], [1, 3], [2, 3]],
            [2, 3],
            super::find_redundant_connection,
        );
        do_unit(
            [[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]],
            [1, 4],
            super::find_redundant_connection,
        );
    }
}
