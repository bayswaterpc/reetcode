use super::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        vec![];
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn max_profit() {
        let nums = vec![7, 1, 5, 3, 6, 4];
        let k = 2;
        assert_eq!([1,2], Solution::top_k_frequent(nums, k));

        let nums = vec![1];
        let k = 2;
        assert_eq!([1], Solution::top_k_frequent(nums));

    }
}
