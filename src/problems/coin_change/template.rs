#[allow(unused)] // remove allow when implementing
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    0
}

pub mod test {

    #[allow(dead_code)]
    // #[test] //comment out above with and run `cargo test unq_test_name`
    fn unit() {
        let candidates = vec![1, 2, 5];
        let amount = 11;
        let out = 3;
        assert_eq!(out, super::coin_change(candidates, amount));

        let candidates = vec![1, 2, 5];
        let amount = 11;
        let out = 3;
        assert_eq!(out, super::coin_change(candidates, amount));
    }
}
