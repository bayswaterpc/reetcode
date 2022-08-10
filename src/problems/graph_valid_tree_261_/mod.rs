use crate::problems::Solution;
pub mod solutions;
pub mod submissions;

impl Solution {
    pub fn graph_valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        solutions::dfs::valid_tree(n, edges)
    }
}
