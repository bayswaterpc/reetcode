use crate::utils::common::OptTreeNode;

// pub type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;

pub fn dfs(node: OptTreeNode, count: &mut i32, mut max_path_val: i32) {
    if let Some(node) = node {
        if node.borrow().val >= max_path_val {
            *count += 1;
            max_path_val = node.borrow().val;
        }
        dfs(node.borrow().left.clone(), count, max_path_val);
        dfs(node.borrow().right.clone(), count, max_path_val);
    }
}

pub fn good_nodes(root: OptTreeNode) -> i32 {
    let mut count = 0;
    let max_path_val = i32::MIN;
    dfs(root, &mut count, max_path_val);
    count
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::count_good_nodes_in_binary_tree_1448_::test::do_unit;

    #[test]
    fn unit() {
        do_unit("3,1,4,3,null,1,5", 4, super::good_nodes);
        do_unit("3,3,null,4,2", 3, super::good_nodes);
    }
}
