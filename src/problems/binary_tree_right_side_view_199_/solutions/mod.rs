use crate::utils::common::OptTreeNode;

// pub type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;

fn dfs(node: OptTreeNode, lvl: usize, rhs: &mut Vec<i32>) {
    if let Some(node) = node {
        if lvl == rhs.len() {
            rhs.push(node.borrow().val);
        }
        dfs(node.borrow().right.clone(), lvl + 1, rhs);
        dfs(node.borrow().left.clone(), lvl + 1, rhs);
    }
}

pub fn right_side_view(root: OptTreeNode) -> Vec<i32> {
    let mut rhs = vec![];
    dfs(root, 0, &mut rhs);
    rhs
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::binary_tree_right_side_view_199_::test::do_unit;

    #[test]
    fn unit() {
        do_unit("1,2,3,null,5,null,4", [1, 3, 4], super::right_side_view);
        do_unit("1,null,3", [1, 3], super::right_side_view);
    }
}
