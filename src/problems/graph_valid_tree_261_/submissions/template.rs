#[allow(unused)] // remove allow when implementing
pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    true
}

pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unit --exact --nocapture
    fn unit() {
        let n = 5;
        let edges = array2d_to_vec2d([[0, 1], [0, 2], [0, 3], [1, 4]]);
        let output = true;
        assert_eq!(output, super::valid_tree(n, edges));

        let n = 5;
        let edges = array2d_to_vec2d([[0, 1], [1, 2], [2, 3], [1, 3], [1, 4]]);
        let output = false;
        assert_eq!(output, super::valid_tree(n, edges));
    }
}
