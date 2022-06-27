use super::Solution;
use crate::utils::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;


// #space=O(1)
// #time=O(N)

type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    /// Solution to [Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/)
    // pub fn is_valid_bst(root: OptTreeNode) -> bool {
    pub fn validate_binary_search(root: OptTreeNode) -> bool {
        fn recurse(node: &OptTreeNode, low: i32, high: i32) -> bool{
            match node {
                None => true,
                Some(nd) => {
                    let nd = nd.borrow();
                    if nd.val > high || nd.val < low {
                        false
                    } else {
                        recurse(&nd.left, low, nd.val) && recurse(&nd.right, nd.val, high)
                    }
                }
            }
        }
        
        recurse(&root, i32::MIN, i32::MAX)
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_str;

    pub fn test_validate_binary_search(tree: &str, is_valid: bool) {
        let root = build_tree_from_lvl_order_str(tree);
        assert_eq!(is_valid, Solution::validate_binary_search(root));
    }

    #[test]
    fn unit() {

        let tree = "2147483647";
        let is_valid = true;
        test_validate_binary_search(tree, is_valid);


        let tree = "2,1,3";
        let is_valid = true;
        test_validate_binary_search(tree, is_valid);

        let tree = "5,1,4,null,null,3,6";
        let is_valid = false;
        test_validate_binary_search(tree, is_valid);

        let tree = "5,4,6,null,null,3,7";
        let is_valid = false;
        test_validate_binary_search(tree, is_valid);

    }
}
