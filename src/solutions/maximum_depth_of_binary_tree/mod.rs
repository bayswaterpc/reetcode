use super::Solution;
use crate::utils::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// #space=O(1)
// #time=O(N)

impl Solution {
    /// Solution to [Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/submissions/)
    //pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    pub fn maximum_depth_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let left = Solution::maximum_depth_of_binary_tree(node.borrow_mut().left.take());
                let right = Solution::maximum_depth_of_binary_tree(node.borrow_mut().right.take());
                left.max(right) + 1
            }
            None => 0,
        }
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_str_list;

    pub fn test_maximum_depth_of_binary_tree(in1: &[&str], output: i32) {
        let in_tree = build_tree_from_lvl_order_str_list(in1);
        assert_eq!(output, Solution::maximum_depth_of_binary_tree(in_tree));
    }

    #[test]
    fn unit() {
        let input = ["3", "9", "20", "null", "null", "15", "7"];
        let output = 3;
        test_maximum_depth_of_binary_tree(&input, output);
    }
}
