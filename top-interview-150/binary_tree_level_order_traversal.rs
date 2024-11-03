//Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).

//Example 1:
//Input: root = [3,9,20,null,null,15,7]
//Output: [[3],[9,20],[15,7]]
//
//Example 2:
//Input: root = [1]
//Output: [[1]]
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

    let result = level_order(Some(p));
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

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    if root.is_none() {
        return res;
    }
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root);
    while !queue.is_empty() {
        let mut level_vals = Vec::new();
        for _ in 0..queue.len() {
            let n = queue.pop_front().unwrap();
            let n = n.unwrap();
            let n = n.borrow();
            level_vals.push(n.val);
            if n.left.clone().is_some() {
                queue.push_back(n.left.clone());
            }
            if n.right.clone().is_some() {
                queue.push_back(n.right.clone());
            }
        }
        res.push(level_vals);
    }
    res
}
