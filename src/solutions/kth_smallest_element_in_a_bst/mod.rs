use super::Solution;
use crate::utils::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    /// Solution to [Kth Smallest Element in a BST](https://leetcode.com/problems/kth-smallest-element-in-a-bst)
    /// #space=Space complexity: O(H)O(H) to keep the stack, where HH is a tree height.
    /// That makes O(N)O(N) in the worst case of the skewed tree, and O(\log N)O(logN) in the average case of the balanced tree.
    /// #time=O(H + k)O(H+k), where HH is a tree height.
    /// This complexity is defined by the stack, which contains at least H + kH+k elements, since before starting to pop out one has to go down to a leaf. This results in O(\log N + k)O(logN+k) for the balanced tree and O(N + k)O(N+k) for completely unbalanced tree with all the nodes in the left subtree.
    //pub fn kth_smallest(root: OptTreeNode, k: i32) -> i32 {
    pub fn kth_smallest_element_in_a_bst(root: OptTreeNode, k: i32) -> i32 {
        let mut k = k;
        let mut stack = vec![];
        let mut curr = root;

        loop {
            while let Some(node) = curr {
                curr = node.borrow_mut().left.take();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                k -= 1;
                if k == 0 {
                    return node.borrow().val;
                }
                curr = node.borrow_mut().right.take();
            } else {
                panic!("BST too small");
            }
        }
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_str;

    pub fn test_kth_smallest_element_in_a_bst(tree: &str, k: i32, out: i32) {
        let root = build_tree_from_lvl_order_str(tree);
        assert_eq!(out, Solution::kth_smallest_element_in_a_bst(root, k));
    }

    #[test]
    fn unit() {
        let tree = "3,1,4,null,2";
        let k = 1;
        let out = 1;
        test_kth_smallest_element_in_a_bst(tree, k, out);

        let tree = "5,3,6,2,4,null,null,1";
        let k = 3;
        let out = 3;
        test_kth_smallest_element_in_a_bst(tree, k, out);
    }
}
