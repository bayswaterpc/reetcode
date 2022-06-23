use super::Solution;

/// [Three Sum](https://leetcode.com/problems/3sum/)
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort();

        for i in 0..nums.len() {
            // Skip repeats to avoid duplicates
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut ll = i + 1;
            let mut rr = nums.len() - 1;

            while ll < rr {
                let sum = nums[i] + nums[ll] + nums[rr];
                match sum.cmp(&0) {
                    std::cmp::Ordering::Less => rr -= 1,
                    std::cmp::Ordering::Equal => ll += 1,
                    std::cmp::Ordering::Greater => {
                        res.push(vec![nums[i], nums[ll], nums[rr]]);
                        ll += 1;

                        // Skip over repeats due to avoid duplicates
                        while nums[ll] == nums[ll - 1] && ll < rr {
                            ll += 1;
                        }
                    }
                }
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
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let mut output = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(output, Solution::three_sum(nums));

        let nums = vec![];
        output = vec![];
        assert_eq!(output, Solution::three_sum(nums));

        let nums = vec![0];
        output = vec![];
        assert_eq!(output, Solution::three_sum(nums));
    }
}
