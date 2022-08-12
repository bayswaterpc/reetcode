#[allow(unused)] //remove unused when implementing
pub fn solve(board: &mut Vec<Vec<char>>) {
    println!("Solve me! {:?}", board);
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::surrounded_regions_190_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(
            [
                ['X', 'X', 'X', 'X'],
                ['X', 'O', 'O', 'X'],
                ['X', 'X', 'O', 'X'],
                ['X', 'O', 'X', 'X'],
            ],
            [
                ['X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X'],
                ['X', 'O', 'X', 'X'],
            ],
            super::solve,
        );
        do_unit([['X']], [['X']], super::solve);
    }
}
