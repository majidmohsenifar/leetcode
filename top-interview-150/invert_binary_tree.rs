//Given the root of a binary tree, invert the tree, and return its root.

//Example 1:
//Input: root = [4,2,7,1,3,6,9]
//Output: [4,7,2,9,6,3,1]
//
//Example 2:
//Input: root = [2,1,3]
//Output: [2,3,1]
//
//Example 3:
//Input: root = []
//Output: []

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    //Input: p = [1,2], q = [1,null,2]
    let p_right = Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    }));
    let p = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(p_right),
    }));

    let result = invert_tree(Some(p));
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

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let node = root.unwrap();
    let inverted_left = invert_tree(node.borrow().left.clone());
    let inverted_right = invert_tree(node.borrow().right.clone());
    let val = node.borrow().val;
    Some(Rc::new(RefCell::new(TreeNode {
        val: val,
        left: inverted_right,
        right: inverted_left,
    })))
}
