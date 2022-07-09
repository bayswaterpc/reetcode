use super::Solution;
use crate::utils::common::ListNode;
// #space=O(1)
// #time=O(N)

impl Solution {
    /// Solution to [Merge Twp Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/submissions/)
    // in leetcode use below def
    // pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>,) -> Option<Box<ListNode>> {
    pub fn merge_two_sorted_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result_head = Box::new(ListNode::new(0));
        let mut curr_node = &mut result_head;
        let mut l1_current = l1;
        let mut l2_current = l2;

        while l1_current.is_some() && l2_current.is_some() {
            // tick here, take content and ownership from l1_current and l2_current, leave l1_current = None, l2_current = None,  l1_current and l2_current must be available after this while block
            let l1_d = l1_current.take();
            let l2_d = l2_current.take();
            if let (Some(mut l1_head), Some(mut l2_head)) = (l1_d, l2_d) {
                if l1_head.val <= l2_head.val {
                    l1_current = l1_head.next.take();
                    // reassign l2_current to get back it's content
                    l2_current = Some(l2_head);
                    curr_node = curr_node.next.get_or_insert(l1_head);
                } else {
                    l2_current = l2_head.next.take();
                    l1_current = Some(l1_head);
                    curr_node = curr_node.next.get_or_insert(l2_head);
                }
            }
        }
        if l1_current.is_some() {
            curr_node.next = l1_current;
        }
        if l2_current.is_some() {
            curr_node.next = l2_current;
        }

        result_head.next
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::make_linked_list;

    pub fn test_merge_two_lists(in1: &[i32], in2: &[i32], output: &[i32]) {
        let l1 = make_linked_list(in1);
        let l2 = make_linked_list(in2);
        let out = make_linked_list(output);
        assert_eq!(out, Solution::merge_two_sorted_lists(l1, l2));
    }

    #[test]
    fn reverse_linked_list() {
        let l1 = [1, 2, 4];
        let l2 = [1, 2, 3];
        let output = [1, 1, 2, 2, 3, 4];
        test_merge_two_lists(&l1, &l2, &output);

        let l1 = [];
        let l2 = [];
        let output = [];
        test_merge_two_lists(&l1, &l2, &output);

        let l1 = [];
        let l2 = [0];
        let output = [0];
        test_merge_two_lists(&l1, &l2, &output);
    }
}
