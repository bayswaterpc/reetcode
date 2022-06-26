use super::Solution;
use crate::utils::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// #space=O(1)
// #time=O(N)

impl Solution {
    /// Solution to [Same Tree](https://leetcode.com/problems/same-tree/)
    //pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    pub fn same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::same_tree_ref(&p, &q)
    }

    pub fn same_tree_ref(
        p: &Option<Rc<RefCell<TreeNode>>>,
        q: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let (p, q) = (p.borrow(), q.borrow());
                p.val == q.val
                    && Solution::same_tree_ref(&p.left, &q.left)
                    && Solution::same_tree_ref(&p.right, &q.right)
            }
            (None, None) => true,
            _ => false,
        }
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_str_list;

    pub fn test_same_tree(p_arr: &[&str], q_arr: &[&str], output: bool) {
        let p = build_tree_from_lvl_order_str_list(p_arr);
        let q = build_tree_from_lvl_order_str_list(q_arr);
        assert_eq!(output, Solution::same_tree(p, q));
    }

    #[test]
    fn same_tree() {
        let p = ["1", "2", "3"];
        let q = ["1", "2", "3"];
        test_same_tree(&p, &q, true);

        let p = ["1", "2"];
        let q = ["1", "null", "2"];
        test_same_tree(&p, &q, false);

        let p = ["1", "2", "1"];
        let q = ["1", "1", "2"];
        test_same_tree(&p, &q, false);
    }
}
