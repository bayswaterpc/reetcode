use super::Solution;

use std::collections::HashSet;
impl Solution {
    /// Solution to [Number of Islands](https://leetcode.com/problems/number-of-islands/submissions/)
    /// time=O(M*N) where m and n are the length and width of the board
    /// space=(M*N) worst case is board full of 1's and a continuous dfs stack covering each cell
    // pub fn pacific_atlantic(mut heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    pub fn pacific_antlantic_water_flow(mut heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(
            set: &mut HashSet<(i32, i32)>,
            arr: &mut Vec<Vec<i32>>,
            r: usize,
            c: usize,
            prev_height: i32,
        ) {
            if prev_height <= arr[r][c] && set.insert((r as i32, c as i32)) {
                // Up
                if r > 0 {
                    dfs(set, arr, r - 1, c, arr[r][c]);
                }
                // Down
                if r < arr.len() - 1 {
                    dfs(set, arr, r + 1, c, arr[r][c]);
                }
                // Left
                if c > 0 {
                    dfs(set, arr, r, c - 1, arr[r][c]);
                }
                // Right
                if c < arr[r].len() - 1 {
                    dfs(set, arr, r, c + 1, arr[r][c]);
                }
            }
        }

        let mut set_pacific = HashSet::default();
        let mut set_atlantic = HashSet::default();

        let num_rows = heights.len() - 1;
        let num_cols = heights[0].len() - 1;

        // We only check for heights on the outer bounds of the `heights` grid
        for c in 0..=num_cols {
            let h_pacific = heights[0][c];
            let h_atlantic = heights[num_rows][c];
            dfs(&mut set_pacific, &mut heights, 0, c, h_pacific);
            dfs(&mut set_atlantic, &mut heights, num_rows, c, h_atlantic);
        }
        for r in 0..=num_rows {
            let h_pacific = heights[r][0];
            let h_atlantic = heights[r][num_cols];
            dfs(&mut set_pacific, &mut heights, r, 0, h_pacific);
            dfs(&mut set_atlantic, &mut heights, r, num_cols, h_atlantic);
        }

        set_atlantic
            .intersection(&set_pacific)
            .into_iter()
            .map(|&(r, c)| vec![r, c])
            .collect()
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    use crate::utils::test_utils::index_set_from_vecs;
    pub fn test_pacific_antlantic_water_flow(heights: Vec<Vec<i32>>, output: Vec<Vec<i32>>) {
        let output = index_set_from_vecs(output);
        let computed = index_set_from_vecs(Solution::pacific_antlantic_water_flow(heights));
        assert_eq!(output, computed);
    }

    #[test]
    fn unit() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let output = vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ];
        test_pacific_antlantic_water_flow(heights, output);

        let heights = vec![vec![2, 1], vec![1, 2]];
        let output = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]];
        test_pacific_antlantic_water_flow(heights, output);
    }
}
