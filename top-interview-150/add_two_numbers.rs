//2. Add Two Numbers
//Medium
//Topics
//Companies
//You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

//You may assume the two numbers do not contain any leading zero, except the number 0 itself.

//Example 1:
//Input: l1 = [2,4,3], l2 = [5,6,4]
//Output: [7,0,8]
//Explanation: 342 + 465 = 807.

//Example 2:
//Input: l1 = [0], l2 = [0]
//Output: [0]

//Example 3:
//Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
//Output: [8,9,9,9,0,0,0,1]
fn main() {
    let l1 = Box::new(ListNode::new(1));
    let l2 = Box::new(ListNode::new(2));
    let result = add_two_numbers(Some(l1), Some(l2));
    println!("result: {:?}", result);
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    add_two_numbers_with_carry(l1, l2, false)
}

pub fn add_two_numbers_with_carry(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    mut carry: bool,
) -> Option<Box<ListNode>> {
    let mut node = match (l1, l2) {
        (Some(n1), Some(n2)) => {
            let mut val = n1.val + n2.val;
            if carry {
                val += 1;
                carry = false;
            }
            if val >= 10 {
                val = val - 10;
                carry = true;
            }
            Some(Box::new(ListNode {
                val,
                next: add_two_numbers_with_carry(n1.next, n2.next, carry),
            }))
        }
        (Some(n), None) => {
            let mut val = n.val;
            if carry {
                val += 1;
                carry = false;
            }
            if val >= 10 {
                val = val - 10;
                carry = true;
            }
            Some(Box::new(ListNode {
                val,
                next: add_two_numbers_with_carry(n.next, None, carry),
            }))
        }
        (None, Some(n)) => {
            let mut val = n.val;
            if carry {
                val += 1;
                carry = false;
            }
            if val >= 10 {
                val = val - 10;
                carry = true;
            }
            Some(Box::new(ListNode {
                val,
                next: add_two_numbers_with_carry(None, n.next, carry),
            }))
        }
        (None, None) => None,
    };
    if node.is_none() && carry {
        node = Some(Box::new(ListNode { val: 1, next: None }));
    }
    node
}
