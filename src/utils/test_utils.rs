use super::common::ListNode;
use super::common::TreeNode;
use std::vec;
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
pub fn build_tree_from_lvl_order_string_list(vals: &[String]) -> Option<Rc<RefCell<TreeNode>>> {
    if vals.len() == 0 || vals[0] == "null" {
        return None;
    }
    let r_val: i32 = vals[0].parse().expect("only valid numeric allowed");
    let root = Rc::new(RefCell::new(TreeNode::new(r_val)));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.clone());
    let mut ii: usize = 1;

    while !queue.is_empty() && ii < vals.len() {
        let node = queue.pop_front().unwrap();
        match vals[ii].as_str() {
            "null" => {}
            _ => {
                let val: i32 = vals[ii].parse().unwrap();
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
        }
        ii += 1;
        if ii >= vals.len(){
            return Some(root);
        }
        match vals[ii].as_str() {
            "null" => {}
            _ => {
                let val: i32 = vals[ii].parse().unwrap();
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().right = Some(right.clone());
                queue.push_back(right);
            }
        }
        ii += 1;
    }
    Some(root)
}


pub fn deserialize(data: String) -> Option<Rc<RefCell<TreeNode>>> {
    let lst: Vec<Option<i32>> = data
        .split(",")
        .map(|s| {
            if s == "null" {
                None
            } else {
                Some(s.parse::<i32>().unwrap())
            }
        })
        .collect();

    let root = match lst[0] {
        Some(a) => Rc::new(RefCell::new(TreeNode::new(a))),
        None => return None,
    };
    let mut cur = 1;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.clone());
    while let Some(node) = queue.pop_front() {
        if let Some(a) = lst[cur] {
            let l = Rc::new(RefCell::new(TreeNode::new(a)));
            node.borrow_mut().left = Some(l.clone());
            queue.push_back(l);
        }
        cur += 1;
        if let Some(a) = lst[cur] {
            let r = Rc::new(RefCell::new(TreeNode::new(a)));
            node.borrow_mut().right = Some(r.clone());
            queue.push_back(r);
        }
        cur += 1;
    }
    Some(root)
}

#[allow(dead_code)]
/// Function to create tree node from list of lvl order nodes
/// Building from strs allows for more easily input null values.
pub fn build_tree_from_lvl_order_str(vals_str: &str) -> Option<Rc<RefCell<TreeNode>>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(vals_str.as_bytes());
    let mut vals = vec![];
    for record in reader.records() {
        let record = record.expect("Should be valid csv cell");
        for field in record.iter() {
            vals.push(field.to_string());
        }
    }
    build_tree_from_lvl_order_string_list(vals.as_slice())
}
