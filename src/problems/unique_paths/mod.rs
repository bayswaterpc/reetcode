use super::Solution;

/// [Unique Paths](https://leetcode.com/problems/unique-paths/)
/// #dp
/// #time=O(M*N)
/// #space=O(M*N)
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![1; n]; m];

        for ii in 1..m {
            for jj in 1..n {
                dp[ii][jj] = dp[ii - 1][jj] + dp[ii][jj - 1];
            }
        }

        dp[m - 1][n - 1]
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let m = 3;
        let n = 7;
        let output = 28;
        assert_eq!(output, Solution::unique_paths(m, n));

        let m = 3;
        let n = 2;
        let output = 3;
        assert_eq!(output, Solution::unique_paths(m, n));
    }
}
