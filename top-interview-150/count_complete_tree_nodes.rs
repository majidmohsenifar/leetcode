//Given the root of a complete binary tree, return the number of the nodes in the tree.

//According to Wikipedia, every level, except possibly the last, is completely filled in a complete binary tree, and all nodes in the last level are as far left as possible. It can have between 1 and 2h nodes inclusive at the last level h.

//Design an algorithm that runs in less than O(n) time complexity.

//Example 1:

//Input: root = [1,2,3,4,5,6]
//Output: 6
//Example 2:

//Input: root = []
//Output: 0
//Example 3:

//Input: root = [1]
//Output: 1

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let result = count_nodes(None);
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

pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let node = root.unwrap();
    let mut nodes = vec![node];
    let mut count = 0;
    while let Some(n) = nodes.pop() {
        count += 1;
        if let Some(l) = n.borrow().left.clone() {
            nodes.push(l);
        }
        if let Some(r) = n.borrow().right.clone() {
            nodes.push(r);
        }
    }
    count
}

pub fn count_nodes_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let node = root.unwrap();
    return 1
        + count_nodes_2(node.borrow().left.clone())
        + count_nodes_2(node.borrow().right.clone());
}
