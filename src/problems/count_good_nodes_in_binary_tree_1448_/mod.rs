use crate::problems::Solution;
pub mod solutions;
pub mod submissions;
use crate::utils::common::OptTreeNode;

impl Solution {
    pub fn count_good_nodes_in_binary_tree_1448_(root: OptTreeNode) -> i32 {
        solutions::good_nodes(root)
    }
}

pub mod test {
    use crate::utils::common::OptTreeNode;
    use crate::utils::test_utils::build_tree_from_lvl_order_str;

    pub fn do_unit<T: Clone + PartialEq + std::fmt::Debug + Sized>(
        input: &str,
        output: T,
        is_balanced: fn(OptTreeNode) -> T,
    ) {
        let root = build_tree_from_lvl_order_str(input);
        assert_eq!(output, is_balanced(root));
    }
}
