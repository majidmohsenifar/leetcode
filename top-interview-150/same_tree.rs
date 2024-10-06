//Given the roots of two binary trees p and q, write a function to check if they are the same or not.

//Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.

//Example 1:

//Input: p = [1,2,3], q = [1,2,3]
//Output: true
//Example 2:

//Input: p = [1,2], q = [1,null,2]
//Output: false
//Example 3:

//Input: p = [1,2,1], q = [1,1,2]
//Output: false
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

    let q_left = Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    }));
    let q = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(q_left),
        right: None,
    }));

    let result = is_same_tree(Some(p), Some(q));
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
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(p_node), Some(q_node)) => {
            let p_node = p_node.borrow();
            let q_node = q_node.borrow();
            if p_node.val != q_node.val {
                return false;
            }
            return is_same_tree(p_node.left.clone(), q_node.left.clone())
                && is_same_tree(p_node.right.clone(), q_node.right.clone());
        }
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (None, None) => true,
    }
}
