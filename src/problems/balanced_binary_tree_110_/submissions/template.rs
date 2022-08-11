use crate::utils::common::OptTreeNode;
// pub type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
#[allow(unused)] //remove unused when implementing
pub fn is_balanced(root: OptTreeNode) -> bool {
    true
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::balanced_binary_tree_110_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit("3,9,20,null,null,15,7", true, super::is_balanced);
        do_unit("1,2,2,3,3,null,null,4,4", false, super::is_balanced);
    }
}
