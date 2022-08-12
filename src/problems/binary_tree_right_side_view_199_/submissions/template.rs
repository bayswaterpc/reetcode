use crate::utils::common::OptTreeNode;
// pub type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
#[allow(unused)] //remove unused when implementing
pub fn right_side_view(root: OptTreeNode) -> Vec<i32> {
    vec![]
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::binary_tree_right_side_view_199_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit("1,2,3,null,5,null,4", [1, 3, 4], super::right_side_view);
        do_unit("1,null,3", [1, 3], super::right_side_view);
    }
}
