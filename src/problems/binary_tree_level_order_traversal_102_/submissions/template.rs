use crate::utils::common::OptTreeNode;

/*********************
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
***********************/

#[allow(unused)] // remove allow when implementing
pub fn binary_tree_level_order_traversal(root: OptTreeNode) -> Vec<Vec<i32>> {
    vec![]
}

pub mod test {
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_str;

    #[allow(dead_code)]
    pub fn test_binary_tree_level_order_traversal(tree: &str, output: Vec<Vec<i32>>) {
        let root = build_tree_from_lvl_order_str(tree);
        assert_eq!(output, super::binary_tree_level_order_traversal(root));
    }

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        let tree = "3,9,20,null,null,15,7";
        let output = vec![vec![3], vec![9, 20], vec![15, 7]];
        test_binary_tree_level_order_traversal(tree, output);
    }
}
