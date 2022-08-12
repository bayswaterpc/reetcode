use crate::utils::common::OptTreeNode;

// pub type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;

// Fills out max_diam and returns longest branch
fn dfs(root: OptTreeNode, max_diam: &mut i32) -> i32 {
    if let Some(ref root) = root {
        let left_depth = dfs(root.borrow().left.clone(), max_diam) + 1;
        let right_depth = dfs(root.borrow().right.clone(), max_diam) + 1;
        *max_diam = std::cmp::max(*max_diam, left_depth + right_depth);
        std::cmp::max(left_depth, right_depth)
    } else {
        -1
    }
}

pub fn diameter_of_binary_tree(root: OptTreeNode) -> i32 {
    let mut max_depth = i32::MIN;
    dfs(root, &mut max_depth);
    max_depth
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::balanced_binary_tree_110_::test::do_unit;

    #[test]
    fn unit() {
        do_unit("1,2,3,4,5", 3, super::diameter_of_binary_tree);
        do_unit("1,2", 1, super::diameter_of_binary_tree);
    }
}
