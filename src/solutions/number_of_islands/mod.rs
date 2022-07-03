use super::Solution;
impl Solution {
    /// Solution to [Number of Islands](https://leetcode.com/problems/number-of-islands/submissions/)
    /// time=O(M*N) where m and n are the length and width of the board
    /// space=(M*N) worst case is board full of 1's and a continuous dfs stack covering each cell
    // pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    pub fn number_of_islands(grid: Vec<Vec<char>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<char>>, ii: usize, jj: usize) -> i32 {
            if ii >= grid.len() || jj >= grid[0].len() {
                return 0;
            }
            if grid[ii][jj] != '1' {
                return 0;
            }
            let mut num_cells = 1;
            grid[ii][jj] = '2';
            for d in [0, 1, 0, !0, 0].windows(2) {
                let (ni, nj) = (ii.wrapping_add(d[0]), jj.wrapping_add(d[1]));
                num_cells += dfs(grid, ni, nj);
            }

            num_cells
        }

        let mut num_islands = 0;
        let mut grid = grid;
        for ii in 0..grid.len() {
            for jj in 0..grid[0].len() {
                if dfs(&mut grid, ii, jj) != 0 {
                    num_islands += 1;
                }
            }
        }

        num_islands
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;

    #[test]
    fn unit() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        let output = 1;
        assert_eq!(output, Solution::number_of_islands(grid));
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        let output = 3;
        assert_eq!(output, Solution::number_of_islands(grid));
    }
}
