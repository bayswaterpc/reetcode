#[allow(unused)] //remove when implementing
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    vec![]
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::spiral_matrix_54_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(
            [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
            [1, 2, 3, 6, 9, 8, 7, 4, 5],
            super::spiral_order,
        );
    }
}
