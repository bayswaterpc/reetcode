#[allow(unused)] //remove unused when implementing
pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    0
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::rotting_oranges_994_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit([[2, 1, 1], [1, 1, 0], [0, 1, 1]], 4, super::oranges_rotting);
        do_unit(
            [[2, 1, 1], [0, 1, 1], [1, 0, 1]],
            -1,
            super::oranges_rotting,
        );
        do_unit([[0, 2]], 0, super::oranges_rotting);
    }
}
