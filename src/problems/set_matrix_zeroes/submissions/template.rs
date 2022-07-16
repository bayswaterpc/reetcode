#[allow(unused)] //remove when implementing
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    println!("Set all rows and columns to zero!! {:?}", matrix);
}

pub mod test {
    #[allow(unused_imports, clippy::ptr_arg)]
    use crate::problems::set_matrix_zeroes::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(
            [[1, 1, 1], [1, 0, 1], [1, 1, 1]],
            [[1, 0, 1], [0, 0, 0], [1, 0, 1]],
            super::set_zeroes,
        );

        do_unit(
            [[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]],
            [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]],
            super::set_zeroes,
        );
    }
}
