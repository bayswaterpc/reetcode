use super::Solution;
use crate::utils::common::ListNode;
// #space=O(1)
// #time=O(N)

impl Solution {
    /// Solution to [Remove Nth Node From End of List](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)
    // in leetcode use below def
    //pub fn remove_nth_from_end(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    pub fn remove_nth_node_from_end_of_list(
        head: Option<Box<ListNode>>,
        n: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut dummy = Box::new(dummy);
        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();
        // move fast n forward
        for _ in 0..n {
            fast = fast.next.unwrap();
        }

        while fast.next.is_some() {
            fast = fast.next.unwrap();
            slow = slow.next.as_mut().unwrap();
        }
        let next = slow.next.as_mut().unwrap();
        slow.next = next.next.clone();
        dummy.next
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::make_linked_list;

    pub fn test_remove_nth_node_from_end_of_list(input: &[i32], n: &i32, output: &[i32]) {
        let in_head = make_linked_list(input);
        let out_ll = make_linked_list(output);
        assert_eq!(
            out_ll,
            Solution::remove_nth_node_from_end_of_list(in_head, *n)
        );
    }

    #[test]
    fn unit() {
        let input = [1, 2, 3, 4, 5];
        let n = 2;
        let output = [1, 2, 3, 5];
        test_remove_nth_node_from_end_of_list(&input, &n, &output);

        let input = [1];
        let n = 1;
        let output = [];
        test_remove_nth_node_from_end_of_list(&input, &n, &output);

        let input = [1, 2];
        let n = 1;
        let output = [1];
        test_remove_nth_node_from_end_of_list(&input, &n, &output);
    }
}
