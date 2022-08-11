#[allow(unused)] //remove unused when implementing
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    1
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::koko_eating_bananas_875_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit([312884470], 968709470, 1, super::min_eating_speed);
        do_unit([3,6,7,11], 8, 4, super::min_eating_speed);
        do_unit([30,11,23,4,20], 5, 30, super::min_eating_speed);
        do_unit([30,11,23,4,20], 6, 23, super::min_eating_speed);
    }
}
