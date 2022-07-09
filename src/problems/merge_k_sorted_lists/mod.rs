use super::Solution;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;



#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

impl Solution {
    /// Solution to [Merge K Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/)
    /// #space=O(1)
    /// #time=O(N*log(k))
    // in leetcode use below def
    // pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    pub fn merge_k_sorted_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut heap: BinaryHeap<Reverse<Box<ListNode>>> = BinaryHeap::new();
        for list in lists.into_iter().flatten() {
            heap.push(Reverse(list));
        }

        let mut dummy = Box::new(ListNode::new(-1));
        let mut cur = &mut dummy;
        while let Some(Reverse(top)) = heap.pop() {
            cur.next = Some(Box::new(ListNode::new(top.val)));
            cur = cur.next.as_mut().unwrap();
            if let Some(next) = top.next {
                heap.push(Reverse(next));
            }
        }
        dummy.next
    }
}

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        self.val.partial_cmp(&other.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

pub mod test {
    #[allow(unused_imports)]
    use super::{Solution, ListNode};


    pub fn make_linked_list(vals: &[i32]) -> Option<Box<ListNode>> {
        let mut result_head = Box::new(ListNode::new(0));
        let mut curr_node = &mut result_head;

        for val in vals.iter() {
            let v_node = Box::new(ListNode::new(*val));
            curr_node = curr_node.next.get_or_insert(v_node);
        }

        result_head.next
    }


    pub fn test_merge_k_sorted_lists(input: Vec<Vec<i32>>, output: &[i32]) {
        let lists: Vec<Option<Box<ListNode>>> = input
            .into_iter()
            .map(|list| make_linked_list(list.as_slice()))
            .collect();
        let out_lists = make_linked_list(output);
        assert_eq!(out_lists, Solution::merge_k_sorted_lists(lists));
    }

    #[test]
    fn unit() {
        let input = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
        let output = [1, 1, 2, 3, 4, 4, 5, 6];
        test_merge_k_sorted_lists(input, &output);
    }
}
