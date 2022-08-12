use crate::utils::common::OptTreeNode;
// pub type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
#[allow(unused)] //remove unused when implementing
pub fn good_nodes(root: OptTreeNode) -> i32 {
    0
}

pub mod test {
    #[allow(unused_imports)]
    use crate::problems::count_good_nodes_in_binary_tree_1448_::test::do_unit;

    #[allow(dead_code)]
    //#[test] //replace allow with test when ready
    // Use VsCode run or debug options or..
    // change name to a unique test name and run `cargo test unq_name` or  run ..
    // cargo test --package reetcode --lib -- problems::{problemName}::submissions::{submissionUuid}::test::unq_test_name --exact --nocapture
    fn unit() {
        do_unit("3,1,4,3,null,1,5", 4, super::good_nodes);
        do_unit("3,3,null,4,2", 3, super::good_nodes);
    }
}
