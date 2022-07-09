use super::Solution;
use crate::utils::common::ListNode;
// #space=O(1)
// #time=O(N)

impl Solution {
    /// Solution <https://leetcode.com/problems/reorder-list/>
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }

        let mut len = 0;
        let mut tmp = head.as_ref();
        while tmp.is_some() {
            len += 1;
            tmp = tmp.unwrap().next.as_ref();
        }

        let len = len / 2;
        let mut node = head.as_mut();
        for _ in 0..len {
            node = node.unwrap().next.as_mut();
        }

        let mut node = node.unwrap().next.take();
        let mut stack = Vec::new();
        while node.is_some() {
            let tmp = node.as_mut().unwrap().next.take();
            stack.push(node);
            node = tmp;
        }

        let mut curr = head.as_mut().unwrap();
        while !stack.is_empty() {
            let mut node = stack.pop().unwrap();
            node.as_mut().unwrap().next = curr.next.take();
            curr.next = node;
            curr = curr.next.as_mut().unwrap().next.as_mut().unwrap();
        }
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::make_linked_list;

    pub fn test_reorder_list(input: &[i32], output: &[i32]) {
        let mut head = make_linked_list(input);
        let output_linked_list = make_linked_list(output);
        Solution::reorder_list(&mut head);
        assert_eq!(output_linked_list, head);
    }

    #[test]
    fn unit() {
        let input = [1, 2, 3, 4];
        let output = [1, 4, 2, 3];
        test_reorder_list(&input, &output);

        let input = [1, 2, 3, 4, 5];
        let output = [1, 5, 2, 4, 3];
        test_reorder_list(&input, &output);
    }
}
