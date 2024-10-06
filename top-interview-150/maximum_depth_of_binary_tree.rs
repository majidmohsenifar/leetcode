//Given the root of a binary tree, return its maximum depth.

//A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

//Example 1:

//Input: root = [3,9,20,null,null,15,7]
//Output: 3
//Example 2:

//Input: root = [1,null,2]
//Output: 2
//
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    //[3,9,20,null,null,15,7]
    //let input = Some(Rc::new(RefCell::new(TreeNode{})))
    let result = max_depth(None);
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

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut depth = 0;
    match root {
        Some(node) => {
            depth += 1;
            let left_depth = max_depth(node.borrow().left.clone());
            let right_depth = max_depth(node.borrow().right.clone());
            let max = std::cmp::max(left_depth, right_depth);
            depth += max;
            depth
        }
        None => 0,
    }
}
