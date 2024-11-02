//You are given the root of a binary tree containing digits from 0 to 9 only.

//Each root-to-leaf path in the tree represents a number.

//For example, the root-to-leaf path 1 -> 2 -> 3 represents the number 123.
//Return the total sum of all root-to-leaf numbers. Test cases are generated so that the answer will fit in a 32-bit integer.

//A leaf node is a node with no children.

//Example 1:
//Input: root = [1,2,3]
//Output: 25
//Explanation:
//The root-to-leaf path 1->2 represents the number 12.
//The root-to-leaf path 1->3 represents the number 13.
//Therefore, sum = 12 + 13 = 25.

//Example 2:
//Input: root = [4,9,0,5,1]
//Output: 1026
//Explanation:
//The root-to-leaf path 4->9->5 represents the number 495.
//The root-to-leaf path 4->9->1 represents the number 491.
//The root-to-leaf path 4->0 represents the number 40.
//Therefore, sum = 495 + 491 + 40 = 1026.

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    //[3,9,20,null,null,15,7]
    //let left = Rc::new(RefCell::new(TreeNode {
    //val: 9,
    //left: None,
    //right: None,
    //}));

    //let right_l = Rc::new(RefCell::new(TreeNode {
    //val: 15,
    //left: None,
    //right: None,
    //}));

    //let right_r = Rc::new(RefCell::new(TreeNode {
    //val: 7,
    //left: None,
    //right: None,
    //}));

    let left = Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: None,
        right: None,
    }));

    let right = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    }));
    let root = Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(left),
        right: Some(right),
    }));
    //let root = Rc::new(RefCell::new(TreeNode {
    //val: 3,
    //left: None,
    //right: None,
    //}));
    let result = sum_numbers(Some(root));
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

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    sum_numbers_with_parent_val(root, 0)
}

pub fn sum_numbers_with_parent_val(node: Option<Rc<RefCell<TreeNode>>>, val: i32) -> i32 {
    if let Some(n) = node {
        let n = n.borrow();
        let n_val = n.val;
        let temp_val = val + n_val;
        let left_sums = sum_numbers_with_parent_val(n.left.clone(), temp_val * 10);
        let right_sums = sum_numbers_with_parent_val(n.right.clone(), temp_val * 10);
        if left_sums + right_sums == 0 {
            return temp_val;
        }
        return left_sums + right_sums;
    }
    0
}
