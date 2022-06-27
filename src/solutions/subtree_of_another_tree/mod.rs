use super::Solution;
use crate::utils::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// #space=O(1)
// #time=O(N)

type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    /// Solution to [Same Tree](https://leetcode.com/problems/same-tree/)
    /// Solution to [Subtree of Another Tree](https://leetcode.com/problems/subtree-of-another-tree/)
    // pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    pub fn subtree_of_another_tree(root: OptTreeNode, sub_root: OptTreeNode) -> bool {
        fn has_match(root: &OptTreeNode, sub: &OptTreeNode) -> bool {
            if Solution::same_tree_ref(root, sub) {
                true
            } else if let Some(ref node) = root {
                let node = node.borrow();
                has_match(&node.left, sub) || has_match(&node.right, sub)
            } else {
                false
            }
        }

        has_match(&root, &sub_root)
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_str;

    pub fn test_subtree_of_another_tree(root_str: &str, subtree_arr: &str, output: bool) {
        let root = build_tree_from_lvl_order_str(root_str);
        let sub_root = build_tree_from_lvl_order_str(subtree_arr);
        assert_eq!(output, Solution::subtree_of_another_tree(root, sub_root));
    }

    #[test]
    fn unit() {
        let tree = "3,4,5,1,2";
        let subtree = "4,1,2";
        test_subtree_of_another_tree(&tree, &subtree, true);

        let tree = "3,4,5,1,2,null,null,null,null,0";
        let subtree = "4,1,2";
        test_subtree_of_another_tree(&tree, &subtree, false);
    }
}
