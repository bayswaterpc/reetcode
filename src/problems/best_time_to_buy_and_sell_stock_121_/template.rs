#[allow(unused)] // remove allow when implementing
pub fn best_time_to_buy_and_sell_stock(prices: Vec<i32>) -> i32 {
    1
}

mod test {
    #[allow(dead_code)]
    //#[test] //replace allow with test when implementing
    fn best_time_to_buy_and_sell_stock() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(5, super::best_time_to_buy_and_sell_stock(prices));
        // Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
        // Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.

        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(0, super::best_time_to_buy_and_sell_stock(prices));
        // Explanation: In this case, no transactions are done and the max profit = 0.
    }
}
