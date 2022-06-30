use super::Solution;

impl Solution {
    ///Solution to [Combination Sum](https://leetcode.com/problems/combination-sum/)
    /// Where N is the number of candidates, T is the target value, and M is the minimal value among the candidates.
    /// #time=(O(N^(T/M+1)))
    /// #space=(O(T/M))
    //pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            candidates: &Vec<i32>,
            rem: i32,
            start: usize,
            res: &mut Vec<Vec<i32>>,
            combo: &mut Vec<i32>,
        ) {
            match rem.cmp(&0) {
                std::cmp::Ordering::Equal => {
                    res.push(combo.clone());
                }
                std::cmp::Ordering::Greater => {
                    for ii in start..candidates.len() {
                        combo.push(candidates[ii]);
                        backtrack(candidates, rem - candidates[ii], ii, res, combo);
                        combo.pop();
                    }
                }
                _ => {}
            }
        }
        let mut res = vec![];
        let mut combo = vec![];
        backtrack(&candidates, target, 0, &mut res, &mut combo);
        res
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let out = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(out, Solution::combination_sum(candidates, target));

        let candidates = vec![2, 3, 5];
        let target = 8;
        let out = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(out, Solution::combination_sum(candidates, target));

        let candidates = vec![2];
        let target = 1;
        let out: Vec<Vec<i32>> = vec![];
        assert_eq!(out, Solution::combination_sum(candidates, target));
    }
}
