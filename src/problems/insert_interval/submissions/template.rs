#[allow(unused)] //remove when implementing
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    vec![]
}



pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::array2d_to_vec2d;

    
    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        let intervals = array2d_to_vec2d([[1,3],[6,9]]);
        let new_interval = vec![2, 5];
        let output = array2d_to_vec2d([[1,5],[6,9]]);
        assert_eq!(output, super::insert(intervals, new_interval));

        let intervals = array2d_to_vec2d([[1,2],[3,5],[6,7],[8,10],[12,16]]);
        let new_interval = vec![4,8];
        let output = array2d_to_vec2d([[1,2],[3,10],[12,16]]);
        assert_eq!(output, super::insert(intervals, new_interval));


        let intervals = array2d_to_vec2d([[1,2],[3,5],[6,7],[8,10],[12,16]]);
        let new_interval = vec![0,20];
        let output = array2d_to_vec2d([[0,20]]);
        assert_eq!(output, super::insert(intervals, new_interval));

    }
}
