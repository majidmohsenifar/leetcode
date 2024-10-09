use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let result = has_path_sum(None, 22);
    println!("result: {:#?}", result)
}

//Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root.is_none() {
        return false;
    }
    let node = root.unwrap();
    let node = node.borrow();
    let left = node.left.clone();
    let right = node.right.clone();
    if node.val == target_sum && left.is_none() && right.is_none() {
        return true;
    }
    return has_path_sum(left, target_sum - node.val) || has_path_sum(right, target_sum - node.val);
}
