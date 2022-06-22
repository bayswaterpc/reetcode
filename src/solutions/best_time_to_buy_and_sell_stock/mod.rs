use super::Solution;

impl Solution {
    /// Solution to [Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)
    // in leetcode use below def
    // pub fn max_profit(prices: Vec<i32>) -> i32 {
    pub fn best_time_to_buy_and_sell_stock(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for p in prices {
            if p < min_price {
                min_price = p;
            } else if p - min_price > max_profit {
                max_profit = p - min_price
            }
        }

        max_profit
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(5, Solution::best_time_to_buy_and_sell_stock(prices));
        // Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
        // Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.

        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(0, Solution::best_time_to_buy_and_sell_stock(prices));
        // Explanation: In this case, no transactions are done and the max profit = 0.
    }
}
