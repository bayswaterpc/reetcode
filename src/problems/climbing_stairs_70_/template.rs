#[allow(unused)] // remove allow when implementing
pub fn climbing_stairs(n: i32) -> i32 {
    0
}

mod test {

    #[allow(dead_code)]
    // #[test] //comment out above with and run `cargo test unq_test_name`
    fn unit() {
        let n = 2;
        let output = 2;
        assert_eq!(output, super::climbing_stairs(n));

        let n = 10;
        let output = 89;
        assert_eq!(output, super::climbing_stairs(n));
    }
}
