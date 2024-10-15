//Given the root of a Binary Search Tree (BST), return the minimum absolute difference between the values of any two different nodes in the tree.

//Example 1:

//Input: root = [4,2,6,1,3]
//Output: 1
//Example 2:

//Input: root = [1,0,48,null,null,12,49]
//Output: 1
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    //[4,2,6,1,3]
    let left_l = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    }));

    let left_r = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    }));

    let left = Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(left_l),
        right: Some(left_r),
    }));
    let right = Rc::new(RefCell::new(TreeNode {
        val: 6,
        left: None,
        right: None,
    }));
    let root = Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(left),
        right: Some(right),
    }));
    let result = get_minimum_difference(Some(root));
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

pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //these act like global variables
    let mut min = i32::MAX;
    let mut prev = None;
    in_order_traverse(root, &mut min, &mut prev);
    min
}

pub fn in_order_traverse(
    node: Option<Rc<RefCell<TreeNode>>>,
    min: &mut i32,
    prev: &mut Option<i32>,
) {
    if let Some(n) = node {
        let n = n.borrow();
        in_order_traverse(n.left.clone(), min, prev);
        if let Some(p) = *prev {
            *min = std::cmp::min(*min, n.val - p);
        }
        *prev = Some(n.val);
        in_order_traverse(n.right.clone(), min, prev);
    }
}
