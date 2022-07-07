use super::Solution;

impl Solution {
    /// Solution to [Coin Change](https://leetcode.com/problems/coin-change/)
    /// #time=O(S*N) where S is the amount and N is the number of coins
    /// #space=O(A) where S is the amount and memoization array
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        fn recurse(coins: &[i32], num_coins_2_amnt: &mut [i32], rem: i32) -> i32 {
            if rem < 0 {
                return -1;
            }
            if rem == 0 {
                return 0;
            }
            let amnt_indx = (rem - 1) as usize;
            if num_coins_2_amnt[amnt_indx] != 0 {
                return num_coins_2_amnt[amnt_indx];
            }

            let mut min_coin = i32::MAX;
            for coin in coins {
                let res = recurse(coins, num_coins_2_amnt, rem - coin);
                if res >= 0 && res < min_coin {
                    min_coin = res + 1;
                }
            }

            num_coins_2_amnt[amnt_indx] = if min_coin == i32::MAX { -1 } else { min_coin };

            num_coins_2_amnt[amnt_indx]
        }

        let mut num_coins_2_amnt = vec![0; amount as usize];
        recurse(&coins, &mut num_coins_2_amnt, amount)
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let candidates = vec![1, 2, 5];
        let amount = 11;
        let out = 3;
        assert_eq!(out, Solution::coin_change(candidates, amount));

        let candidates = vec![1, 2, 5];
        let amount = 11;
        let out = 3;
        assert_eq!(out, Solution::coin_change(candidates, amount));
    }
}
