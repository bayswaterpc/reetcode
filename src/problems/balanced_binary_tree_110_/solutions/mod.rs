use crate::utils::common::OptTreeNode;
// pub type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
fn get_balanced_maxh(root: &OptTreeNode) -> (bool, i32) {
    if let Some(r) = root {
        let (lb, lh) = get_balanced_maxh(&r.borrow().left);
        let (rb, rh) = get_balanced_maxh(&r.borrow().right);
        let maxh = rh.max(lh) + 1;
        if !lb || !rb || (rh - lh).abs() > 1 {
            return (false, maxh);
        }
        (true, maxh)
    } else {
        (true, 0)
    }
}

pub fn is_balanced(root: OptTreeNode) -> bool {
    get_balanced_maxh(&root).0
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::balanced_binary_tree_110_::test::do_unit;

    #[test]
    fn unit() {
        do_unit("3,9,20,null,null,15,7", true, super::is_balanced);
        do_unit("1,2,2,3,3,null,null,4,4", false, super::is_balanced);
    }
}
