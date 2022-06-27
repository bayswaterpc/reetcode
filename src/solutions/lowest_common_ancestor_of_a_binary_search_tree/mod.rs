use super::Solution;
use crate::utils::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// #space=O(1)
// #time=O(N)

type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    /// Solution to [Lowest Common Ancestor of a Binary Search Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)
    // pub fn lowest_common_ancestor(
    pub fn lowest_common_ancestor_of_a_binary_search_tree(
        root: OptTreeNode,
        p: OptTreeNode,
        q: OptTreeNode,
    ) -> OptTreeNode {
        let p_val = p.as_ref().unwrap().borrow().val;
        let q_val = q.as_ref().unwrap().borrow().val;
        let (min, max) = match p_val > q_val {
            true => (q_val, p_val),
            false => (p_val, q_val),
        };

        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if let Some(node) = node {
                let node_val = node.borrow().val;
                if node_val >= min && node_val <= max {
                    return Some(node);
                }
                match node_val > max {
                    true => stack.push(node.borrow().left.clone()),
                    false => stack.push(node.borrow().right.clone()),
                }
            }
        }
        unreachable!()
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_str;

    pub fn test_lowest_common_ancestor_of_a_binary_search_tree(
        tree: &str,
        p_str: &str,
        q_str: &str,
        out: i32,
    ) {
        let root = build_tree_from_lvl_order_str(tree);
        let p = build_tree_from_lvl_order_str(p_str);
        let q = build_tree_from_lvl_order_str(q_str);

        let lca = Solution::lowest_common_ancestor_of_a_binary_search_tree(root, p, q);

        if let Some(node) = lca {
            assert_eq!(node.borrow().val, out);
        } else {
            panic!("Should always be in tree")
        }
    }

    #[test]
    fn unit() {
        let tree = "6,2,8,0,4,7,9,null,null,3,5";
        let p_str = "2";
        let q_str = "8";
        let out = 6;
        test_lowest_common_ancestor_of_a_binary_search_tree(tree, p_str, q_str, out);

        let tree = "6,2,8,0,4,7,9,null,null,3,5";
        let p_str = "2";
        let q_str = "4";
        let out = 2;
        test_lowest_common_ancestor_of_a_binary_search_tree(tree, p_str, q_str, out);

        let tree = "2,1";
        let p_str = "2";
        let q_str = "1";
        let out = 2;
        test_lowest_common_ancestor_of_a_binary_search_tree(tree, p_str, q_str, out);
    }
}
