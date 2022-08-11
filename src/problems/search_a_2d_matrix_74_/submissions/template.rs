#[allow(unused)] //remove unused when implementing
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    false
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::search_a_2d_matrix_74_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(
            [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]],
            3,
            true,
            super::search_matrix,
        );
        do_unit(
            [[1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 60]],
            3,
            true,
            super::search_matrix,
        );
    }
}
