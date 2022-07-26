use crate::utils::common::OptTreeNode;

/***************
use std::{cell::RefCell, rc::Rc};
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
// pub type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
***************/

#[allow(unused)] // remove allow when implementing
pub fn binary_tree_maximum_path_sum(root: OptTreeNode) -> i32 {
    0
}

pub mod test {
    #[allow(unused_imports)]
    use super::super::solution::test::test_binary_tree_maximum_path_sum;

    #[allow(dead_code)]
    // #[test] //comment out above with and run `cargo test unq_test_name`
    fn unit() {
        let tree = "1,2,3";
        let out = 6;
        test_binary_tree_maximum_path_sum(tree, out);

        let tree = "-10,9,20,null,null,15,7";
        let out = 42;
        test_binary_tree_maximum_path_sum(tree, out);
    }
}
