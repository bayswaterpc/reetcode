use super::common::ListNode;
use super::common::TreeNode;
use std::{cell::RefCell, rc::Rc};

#[allow(dead_code)]
pub fn str_vec_to_string_vec(str_vec: Vec<&str>) -> Vec<String> {
    str_vec.into_iter().map(|str| (*str).to_string()).collect()
}

#[allow(dead_code)]
pub fn str_vec_2d_to_string_vec_2d(str_vec_2d: Vec<Vec<&str>>) -> Vec<Vec<String>> {
    str_vec_2d.into_iter().map(str_vec_to_string_vec).collect()
}

#[allow(dead_code)]
pub fn str_vec_2d_contents_same(strs1: Vec<Vec<String>>, strs2: Vec<Vec<String>>) -> bool {
    let mut sorted_strs1: Vec<String> = strs1
        .into_iter()
        .map(|mut strs| {
            strs.sort();
            strs.join("")
        })
        .collect();
    let mut sorted_strs2: Vec<String> = strs2
        .into_iter()
        .map(|mut strs| {
            strs.sort();
            strs.join("")
        })
        .collect();
    sorted_strs1.sort();
    sorted_strs2.sort();
    sorted_strs1 == sorted_strs2
}

#[allow(dead_code)]
pub fn make_linked_list(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut result_head = Box::new(ListNode::new(0));
    let mut curr_node = &mut result_head;

    for val in vals.iter() {
        let v_node = Box::new(ListNode::new(*val));
        curr_node = curr_node.next.get_or_insert(v_node);
    }

    result_head.next
}

#[allow(dead_code)]
/// Function to create tree node from list of lvl order nodes
pub fn build_tree_from_lvl_order_list(vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    fn recurse(vals: &[i32], indx: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if indx >= vals.len() {
            return None;
        }

        let node = Rc::new(RefCell::new(TreeNode::new(vals[indx])));
        node.borrow_mut().left = recurse(vals, 2 * indx + 1);
        node.borrow_mut().right = recurse(vals, 2 * indx + 2);

        Some(node)
    }
    recurse(vals, 0)
}

#[allow(dead_code)]
/// Function to create tree node from list of lvl order nodes
/// Building from strs allows for more easily input null values.
pub fn build_tree_from_lvl_order_str_list(vals: &[&str]) -> Option<Rc<RefCell<TreeNode>>> {
    fn recurse(vals: &[&str], indx: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if indx >= vals.len() || vals[indx] == "null" {
            return None;
        }
        let val: i32 = vals[indx]
            .parse()
            .expect("Must always be valid num or null");
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        node.borrow_mut().left = recurse(vals, 2 * indx + 1);
        node.borrow_mut().right = recurse(vals, 2 * indx + 2);

        Some(node)
    }
    recurse(vals, 0)
}
