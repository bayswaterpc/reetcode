use super::Solution;
use crate::utils::common::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    /// Solution to [Construct Binary Tree from Preorder and Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)
    /// #space=O(N)
    /// #time=O(N)
    /// #binary-tree
    // pub fn build_tree(
    pub fn construct_binary_tree_from_preorder_and_inorder_traversal(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
    ) -> OptTreeNode {
        let inorder_indx_map = inorder
            .iter()
            .enumerate()
            .map(|(i, &val)| (val, i))
            .collect::<HashMap<_, _>>();
        Self::helper(
            &mut preorder.iter(),
            &inorder_indx_map,
            (0, preorder.len() as isize - 1),
        )
    }
    fn helper(
        preorder: &mut std::slice::Iter<i32>,
        inorder_indx_map: &HashMap<i32, usize>,
        range: (isize, isize),
    ) -> OptTreeNode {
        if range.0 <= range.1 {
            if let Some(&val) = preorder.next() {
                if let Some(&ii) = inorder_indx_map.get(&val) {
                    return Some(Rc::new(RefCell::new(TreeNode {
                        val,
                        left: Self::helper(preorder, inorder_indx_map, (range.0, ii as isize - 1)),
                        // Do not forget preorder will be iterated by above call on left
                        right: Self::helper(preorder, inorder_indx_map, (ii as isize + 1, range.1)),
                    })));
                }
            }
        }
        None
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_str;

    pub fn test_construct_binary_tree_from_preorder_and_inorder_traversal(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
        out: &str,
    ) {
        let tree =
            Solution::construct_binary_tree_from_preorder_and_inorder_traversal(preorder, inorder);
        let out_tree = build_tree_from_lvl_order_str(out);
        assert_eq!(tree, out_tree);
    }

    #[test]
    fn unit() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let out = "3,9,20,null,null,15,7";
        test_construct_binary_tree_from_preorder_and_inorder_traversal(preorder, inorder, out);

        let preorder = vec![-1];
        let inorder = vec![-1];
        let out = "-1";
        test_construct_binary_tree_from_preorder_and_inorder_traversal(preorder, inorder, out);
    }
}
