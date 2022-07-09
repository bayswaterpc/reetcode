use super::Solution;
use std::collections::BinaryHeap;
use std::collections::HashMap;
impl Solution {
    /// Solution to [Top K Frequent Elements](https://leetcode.com/problems/top-k-frequent-elements/)
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hashmap = HashMap::new();
        for num in nums {
            *hashmap.entry(num).or_insert(0) += 1;
        }
        let freq_num: Vec<(i32, i32)> = hashmap.into_iter().map(|(a, b)| (b, a)).collect();

        let mut res = Vec::new();
        let mut maxheap: BinaryHeap<(i32, i32)> = BinaryHeap::from(freq_num);

        while let Some((_, num)) = maxheap.pop() {
            res.push(num);
            if res.len() == k as usize {
                break;
            }
        }
        res
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        assert_eq!(vec![1, 2], Solution::top_k_frequent(nums, k));

        let nums = vec![1];
        let k = 2;
        assert_eq!(vec![1], Solution::top_k_frequent(nums, k));
    }
}
