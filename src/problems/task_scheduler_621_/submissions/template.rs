#[allow(unused)] //remove unused when implementing
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    0
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::task_scheduler_621_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(['A', 'A', 'A', 'B', 'B', 'B'], 2, 8, super::least_interval);
        do_unit(['A', 'A', 'A', 'B', 'B', 'B'], 0, 6, super::least_interval);
        do_unit(
            ['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
            2,
            16,
            super::least_interval,
        );
    }
}
