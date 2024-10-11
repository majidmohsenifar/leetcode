//Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).

//Example 1:

//Input: root = [1,2,2,3,4,4,3]
//Output: true
//Example 2:

//Input: root = [1,2,2,null,3,null,3]
//Output: false
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
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

    let result = is_symmetric(Some(p));
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
//using stack
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }
    let node = root.unwrap();
    let mut nodes = vec![(node.borrow().left.clone(), node.borrow().right.clone())];
    while let Some((l, r)) = nodes.pop() {
        match (l, r) {
            (Some(n1), Some(n2)) => {
                if n1.borrow().val != n2.borrow().val {
                    return false;
                }
                nodes.push((n1.borrow().left.clone(), n2.borrow().right.clone()));
                nodes.push((n1.borrow().right.clone(), n2.borrow().left.clone()));
                continue;
            }
            (Some(_), None) => return false,
            (None, Some(_)) => return false,
            (None, None) => {}
        }
    }
    true
}

//using recursive
pub fn is_symmetric_2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }
    let node = root.unwrap();
    return are_equal(node.borrow().left.clone(), node.borrow().right.clone());
}

fn are_equal(n1: Option<Rc<RefCell<TreeNode>>>, n2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (n1, n2) {
        (Some(n1), Some(n2)) => {
            if n1.borrow().val != n2.borrow().val {
                return false;
            }
            return are_equal(n1.borrow().left.clone(), n2.borrow().right.clone())
                && are_equal(n1.borrow().right.clone(), n2.borrow().left.clone());
        }
        (Some(_), None) => return false,
        (None, Some(_)) => return false,
        (None, None) => return true,
    }
}
