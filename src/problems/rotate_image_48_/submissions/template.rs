#[allow(unused)] //remove when implementing
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    println!("rotate me! {:?}", matrix);
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::rotate_image_48_::test::test_rotate;

    #[allow(dead_code)]
    // #[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        test_rotate(
            [[1, 2, 3], [4, 5, 6], [7, 8, 9]],
            [[7, 4, 1], [8, 5, 2], [9, 6, 3]],
            super::rotate,
        );
        test_rotate(
            [
                [5, 1, 9, 11],
                [2, 4, 8, 10],
                [13, 3, 6, 7],
                [15, 14, 12, 16],
            ],
            [
                [15, 13, 2, 5],
                [14, 3, 4, 1],
                [12, 6, 8, 9],
                [16, 7, 10, 11],
            ],
            super::rotate,
        );
    }
}
