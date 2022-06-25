use super::Solution;
use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::common::TreeNode;

// #space=O(1)
// #time=O(N)

impl Solution {
    /// Solution to [Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/)
    //pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    pub fn invert_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let right = node.borrow_mut().right.take();
            let left = node.borrow_mut().left.take();
            node.borrow_mut().left = Self::invert_binary_tree(right);
            node.borrow_mut().right = Self::invert_binary_tree(left);
            return Some(node);
        } else {
            return None;
        }
    }
}


pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_list;

    pub fn test_merge_two_lists(in1: &[i32], output: &[i32]) {
        let in_tree = build_tree_from_lvl_order_list(in1);
        let out_tree = build_tree_from_lvl_order_list(output);
        assert_eq!(out_tree, Solution::invert_binary_tree(in_tree));
    }

    #[test]
    fn invert_tree() {
        let input = [4,2,7,1,3,6,9];
        let output = [4,7,2,9,6,3,1];
        test_merge_two_lists(&input, &output);
    }
}
