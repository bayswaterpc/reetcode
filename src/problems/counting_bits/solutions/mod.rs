use crate::problems::Solution;

pub fn count_bits(n: i32) -> Vec<i32> {
    (0..((n + 1) as u32))
        .into_iter()
        .map(Solution::number_of_1_bits)
        .collect()
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::counting_bits::test::do_unit;

    #[test]
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit(2, [0, 1, 1], super::count_bits);
        do_unit(5, [0, 1, 1, 2, 1, 2], super::count_bits);
    }
}
