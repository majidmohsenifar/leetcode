//You are given the heads of two sorted linked lists list1 and list2.

//Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.

//Return the head of the merged linked list.

//Example 1:
//Input: list1 = [1,2,4], list2 = [1,3,4]
//Output: [1,1,2,3,4,4]

//Example 2:
//Input: list1 = [], list2 = []
//Output: []

//Example 3:
//Input: list1 = [], list2 = [0]
//Output: [0]

fn main() {
    let list1 = Box::new(ListNode::new(1));
    let list2 = Box::new(ListNode::new(2));
    let result = merge_two_lists(Some(list1), Some(list2));
    println!("result: {:#?}", result)
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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut node = ListNode { val: 0, next: None };
    match (list1, list2) {
        (Some(l1), Some(l2)) => {
            if l1.val < l2.val {
                node.val = l1.val;
                node.next = merge_two_lists(l1.next, Some(l2));
            } else {
                node.val = l2.val;
                node.next = merge_two_lists(Some(l1), l2.next);
            }
        }
        (Some(l1), None) => return Some(l1),
        (None, Some(l2)) => return Some(l2),
        (None, None) => return None,
    }
    Some(Box::new(node))
}
