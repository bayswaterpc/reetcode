use crate::problems::Solution;
use crate::utils::common::OptTreeNode;

impl Solution {
    /// Solution to [Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/)
    /// #space=O(1)
    /// #time=O(N)
    //pub fn level_order(root: OptTreeNode) -> Vec<Vec<i32>> {
    pub fn binary_tree_level_order_traversal(root: OptTreeNode) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        fn recurse(root: &OptTreeNode, res: &mut Vec<Vec<i32>>, level: usize) {
            if let Some(r) = root {
                if res.len() == level {
                    res.push(Vec::new());
                }
                res[level].push(r.borrow().val);
                recurse(&r.borrow().left, res, level + 1);
                recurse(&r.borrow().right, res, level + 1);
            }
        }
        recurse(&root, &mut res, 0);

        res
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_str;

    pub fn test_binary_tree_level_order_traversal(tree: &str, lvls: Vec<Vec<i32>>) {
        let root = build_tree_from_lvl_order_str(tree);
        assert_eq!(lvls, Solution::binary_tree_level_order_traversal(root));
    }

    #[test]
    fn unit() {
        let tree = "3,9,20,null,null,15,7";
        let lvls = vec![vec![3], vec![9, 20], vec![15, 7]];
        test_binary_tree_level_order_traversal(tree, lvls);
    }
}
