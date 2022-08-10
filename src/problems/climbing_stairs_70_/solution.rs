use crate::problems::Solution;

impl Solution {
    /// Solution to [Climbing Stairs](https://leetcode.com/problems/climbing-stairs/)
    /// #time=(O(N))
    /// #space=(O(N))
    //pub fn climb_stairs(n: i32) -> i32 {
    pub fn climbing_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut ways_to_step = vec![0; n + 1];
        ways_to_step[0] = 1;
        ways_to_step[1] = 1;

        for ii in 2..n + 1 {
            ways_to_step[ii] = ways_to_step[ii - 1] + ways_to_step[ii - 2]
        }

        ways_to_step[n]
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let n = 2;
        let output = 2;
        assert_eq!(output, Solution::climbing_stairs(n));

        let n = 10;
        let output = 89;
        assert_eq!(output, Solution::climbing_stairs(n));
    }
}
