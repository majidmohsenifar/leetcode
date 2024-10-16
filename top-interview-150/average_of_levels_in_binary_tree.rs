//Given the root of a binary tree, return the average value of the nodes on each level in the form of an array. Answers within 10-5 of the actual answer will be accepted.

//Example 1:
//Input: root = [3,9,20,null,null,15,7]
//Output: [3.00000,14.50000,11.00000]
//Explanation: The average value of nodes on level 0 is 3, on level 1 is 14.5, and on level 2 is 11.
//Hence return [3, 14.5, 11].
//
//Example 2:
//Input: root = [3,9,20,15,7]
//Output: [3.00000,14.50000,11.00000]

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    //[3,9,20,null,null,15,7]
    let left = Rc::new(RefCell::new(TreeNode {
        val: 9,
        left: None,
        right: None,
    }));

    let right_l = Rc::new(RefCell::new(TreeNode {
        val: 15,
        left: None,
        right: None,
    }));

    let right_r = Rc::new(RefCell::new(TreeNode {
        val: 7,
        left: None,
        right: None,
    }));

    let right = Rc::new(RefCell::new(TreeNode {
        val: 20,
        left: Some(right_l),
        right: Some(right_r),
    }));
    let root = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(left),
        right: Some(right),
    }));
    let result = average_of_levels(Some(root));
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

pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    let mut averages = vec![];
    let mut nodes = vec![root];
    while !nodes.is_empty() {
        let (mut sum, count) = (0.0, nodes.len());
        let mut level_nodes = vec![];
        for _ in 0..count {
            if let Some(node) = nodes.pop() {
                if let Some(n) = node {
                    let n = n.borrow();
                    sum += n.val as f64;
                    let left = n.left.clone();
                    let right = n.right.clone();
                    if let Some(l) = left {
                        level_nodes.push(l);
                    }
                    if let Some(r) = right {
                        level_nodes.push(r);
                    }
                }
            }
        }
        averages.push(sum / count as f64);
        for n in level_nodes {
            nodes.push(Some(n));
        }
    }
    averages
}
