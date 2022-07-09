use super::Solution;

impl Solution {
    ///Solution to [Combination Sum III](https://leetcode.com/problems/combination-sum-iii/)
    /// #time=(O(9!*k)/(9-k)!)
    /// #space=(O(k))
    //pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    pub fn combination_sum_iii(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        fn backtrack(
            combo: &mut Vec<i32>,
            remainder: i32,
            start: usize,
            res: &mut Vec<Vec<i32>>,
            k: &i32,
        ) {
            if combo.len() == *k as usize && remainder == 0 {
                res.push(combo.clone());
                return;
            } else if combo.len() == *k as usize || remainder < 0 {
                return;
            }

            for ii in start..9 {
                combo.push((ii + 1) as i32);
                let rem = remainder - combo.last().unwrap();
                backtrack(combo, rem, ii + 1, res, k);
                combo.pop();
            }
        }
        let mut combo = vec![];
        backtrack(&mut combo, n, 0, &mut res, &k);
        res
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let k = 3;
        let n = 7;
        let out = vec![vec![1, 2, 4]];
        assert_eq!(out, Solution::combination_sum_iii(k, n));

        let k = 3;
        let n = 9;
        let out = vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]];
        assert_eq!(out, Solution::combination_sum_iii(k, n));

        let k = 4;
        let n = 1;
        let out: Vec<Vec<i32>> = vec![];
        assert_eq!(out, Solution::combination_sum_iii(k, n));
    }
}
