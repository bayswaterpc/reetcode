use super::Solution;
use crate::utils::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type OptTreeNode = Option<Rc<RefCell<TreeNode>>>;
/// Solution to [Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)
/// #space=O(N)
/// #time=O(N)
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    pub fn new() -> Codec {
        Codec {}
    }

    pub fn serialize(&self, root: OptTreeNode) -> String {
        let mut res: Vec<String> = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);

        while let Some(entity) = queue.pop_front() {
            if let Some(ref node) = entity {
                res.push(node.borrow().val.to_string());
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            } else {
                res.push(String::from("null"));
            }
        }
        while !res.is_empty() {
            if res.last().unwrap().as_str() == "null" {
                res.pop();
            } else {
                break;
            }
        }
        res.join(",")
    }

    pub fn deserialize(&self, data: String) -> OptTreeNode {
        println!("data {}", data);
        let lst: Vec<Option<i32>> = data
            .split(',')
            .map(|s| match Some(s.parse::<i32>()) {
                Some(res) => match res {
                    Ok(val) => Some(val),
                    Err(_) => None,
                },
                None => None,
            })
            .collect();

        let root = match lst[0] {
            Some(a) => Rc::new(RefCell::new(TreeNode::new(a))),
            None => return None,
        };
        let mut cur = 1;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        while !queue.is_empty() && cur < lst.len() {
            let node = queue.pop_front().unwrap();
            if let Some(a) = lst[cur] {
                let l = Rc::new(RefCell::new(TreeNode::new(a)));
                node.borrow_mut().left = Some(l.clone());
                queue.push_back(l);
            }
            cur += 1;
            if cur >= lst.len() {
                return Some(root);
            }
            if let Some(a) = lst[cur] {
                let r = Rc::new(RefCell::new(TreeNode::new(a)));
                node.borrow_mut().right = Some(r.clone());
                queue.push_back(r);
            }
            cur += 1;
        }
        Some(root)
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
pub mod test {
    use super::Codec;
    #[allow(unused_imports)]
    use super::Solution;
    #[allow(unused_imports)]
    use crate::utils::test_utils::build_tree_from_lvl_order_str;

    pub fn test_serialize_and_deserialize_binary_tree(input: &str) {
        let root = build_tree_from_lvl_order_str(input);
        let codec = Codec::new();
        let serialized = codec.serialize(root.clone());
        assert_eq!(input, serialized.as_str());

        let deserialized = codec.deserialize(serialized);
        assert_eq!(root, deserialized);
    }

    #[test]
    fn unit() {
        let input = "1,2,3,null,null,4,5";
        test_serialize_and_deserialize_binary_tree(input);

        let input = "";
        test_serialize_and_deserialize_binary_tree(input);
    }
}
