use crate::problems::Solution;
pub mod solutions;
pub mod submissions;
use crate::utils::common::OptTreeNode;

impl Solution {
    pub fn binary_tree_right_side_view_199_(root: OptTreeNode) -> Vec<i32> {
        solutions::right_side_view(root)
    }
}

pub mod test {
    use crate::utils::common::OptTreeNode;
    use crate::utils::test_utils::build_tree_from_lvl_order_str;

    pub fn do_unit<const N: usize, T: Clone + PartialEq + std::fmt::Debug + Sized>(
        input: &str,
        output: [T; N],
        is_balanced: fn(OptTreeNode) -> Vec<T>,
    ) {
        let root = build_tree_from_lvl_order_str(input);
        assert_eq!(output.to_vec(), is_balanced(root));
    }
}
