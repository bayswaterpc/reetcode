use super::Solution;

impl Solution {
    /// Solution to [Longest Increasing Subsequence](https://leetcode.com/problems/longest-increasing-subsequence/submissions/)
    // longest_increasing_subsequence(nums: Vec<i32>) -> i32 {
    pub fn longest_increasing_subsequence(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold(Vec::new(), |mut lis: Vec<i32>, n| {
                let i = lis.binary_search(&n).unwrap_or_else(|x| x);
                if i == lis.len() {
                    lis.push(n);
                } else {
                    lis[i] = n;
                }
                lis
            })
            .len() as i32
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let output = 4;
        assert_eq!(output, Solution::longest_increasing_subsequence(nums));

        let nums = vec![0, 1, 0, 3, 2, 3];
        let output = 4;
        assert_eq!(output, Solution::longest_increasing_subsequence(nums));

        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        let output = 1;
        assert_eq!(output, Solution::longest_increasing_subsequence(nums));
    }
}
