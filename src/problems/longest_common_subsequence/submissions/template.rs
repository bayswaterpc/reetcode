#[allow(unused)] // remove allow when implementing
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    1
}

pub mod test {
    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unit --exact --nocapture
    fn unit() {
        let text1 = "abcde".to_string();
        let text2 = "ace".to_string();
        let output = 3;
        assert_eq!(output, super::longest_common_subsequence(text1, text2));

        let text1 = "abc".to_string();
        let text2 = "abc".to_string();
        let output = 3;
        assert_eq!(output, super::longest_common_subsequence(text1, text2));

        let text1 = "abc".to_string();
        let text2 = "def".to_string();
        let output = 0;
        assert_eq!(output, super::longest_common_subsequence(text1, text2));

        let text1 = "bl".to_string();
        let text2 = "yby".to_string();
        let output = 1;
        assert_eq!(output, super::longest_common_subsequence(text1, text2));
    }
}
