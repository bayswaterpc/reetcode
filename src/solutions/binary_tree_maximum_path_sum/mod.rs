use super::Solution;
use crate::utils::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// #space=O(1)
// #time=O(N)

type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    /// Solution to [Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum/submissions/)
    /// #time=O(log(N))
    /// #space=O(log(H)) where H is the height of the tree
    /// #binary-tree
    //pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    pub fn binary_tree_maximum_path_sum(root: OptTreeNode) -> i32 {
        let (_, max) = Solution::dfs(&root);
        max
    }

    // Return maxBranch &
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(n) = node {
            let (left_val, glbl_l_max) = Solution::dfs(&n.borrow().left);
            let (right_val, glbl_r_max) = Solution::dfs(&n.borrow().right);
            let val = n.borrow().val;

            let mut max_path = left_val.max(right_val);
            max_path = val.max(val + max_path);

            let glbl_n_max = max_path.max(left_val + right_val + val);
            let glbl_max = glbl_n_max.max(glbl_l_max).max(glbl_r_max);

            return (max_path, glbl_max);
        }

        (-100000, -100000)
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_str;

    pub fn test_binary_tree_maximum_path_sum(tree: &str, out: i32) {
        let root = build_tree_from_lvl_order_str(tree);
        assert_eq!(out, Solution::binary_tree_maximum_path_sum(root));
    }

    #[test]
    fn unit() {
        let tree = "1,2,3";
        let out = 6;
        test_binary_tree_maximum_path_sum(tree, out);

        let tree = "-10,9,20,null,null,15,7";
        let out = 42;
        test_binary_tree_maximum_path_sum(tree, out);
    }
}
