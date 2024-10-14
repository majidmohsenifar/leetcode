//Given the root of a binary tree, determine if it is a valid binary search tree (BST).

//A valid BST is defined as follows:

//The left
//subtree
//of a node contains only nodes with keys less than the node's key.
//The right subtree of a node contains only nodes with keys greater than the node's key.
//Both the left and right subtrees must also be binary search trees.
//
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    //[5,4,6,null,null,3,7]
    let left = Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: None,
        right: None,
    }));
    let right_l = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    }));
    let right_r = Rc::new(RefCell::new(TreeNode {
        val: 7,
        left: None,
        right: None,
    }));

    let right = Rc::new(RefCell::new(TreeNode {
        val: 6,
        left: Some(right_l),
        right: Some(right_r),
    }));
    let root = Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(left),
        right: Some(right),
    }));
    let result = is_valid_bst(Some(root));
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

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    is_node_in_range(root, i32::MIN as i64 - 1, i32::MAX as i64 + 1)
}

pub fn is_node_in_range(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
    match node {
        Some(n) => {
            let n = n.borrow();
            if n.val as i64 <= min || n.val as i64 >= max {
                return false;
            }
            return is_node_in_range(n.left.clone(), min, n.val as i64)
                && is_node_in_range(n.right.clone(), n.val as i64, max);
        }
        None => return true,
    }
}
