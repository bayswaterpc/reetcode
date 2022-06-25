use super::Solution;
use crate::utils::common::ListNode;
use std::mem;
// #space=O(1)
// #time=O(N)

impl Solution {
    /// Solution to [Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/)
    // in leetcode use below def
    //pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    pub fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut box_node: Option<Box<ListNode>> = head;
        let mut prev: Option<Box<ListNode>> = None;
        while let Some(mut node) = box_node {
            box_node = mem::replace(&mut node.next, prev);
            prev = Some(node);
        }
        prev
    }
}

mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::make_linked_list;

    #[test]
    fn unit() {
        let input = [1, 2, 3, 4, 5];
        let output = [5, 4, 3, 2, 1];
        let head = make_linked_list(&input);
        let output_linked_list = make_linked_list(&output);
        assert_eq!(output_linked_list, Solution::reverse_linked_list(head));

        let input = [1, 2];
        let output = [2, 1];
        let head = make_linked_list(&input);
        let output_linked_list = make_linked_list(&output);
        assert_eq!(output_linked_list, Solution::reverse_linked_list(head));

        let input = [];
        let output = [];
        let head = make_linked_list(&input);
        let output_linked_list = make_linked_list(&output);
        assert_eq!(output_linked_list, Solution::reverse_linked_list(head));
    }
}
