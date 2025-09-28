use leetcode::ListNode;
use std::boxed::Box;

struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    fn merge_two_lists(&self, l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // dummy head node
        let mut dummy = ListNode::new(-1);
        let mut p = &mut dummy;
        let mut p1 = l1;
        let mut p2 = l2;

        while p1.is_some() && p2.is_some() {
            // compare two pointers p1 and p2
            // attach the node with the smaller value to the p pointer
            if p1.as_ref().unwrap().val > p2.as_ref().unwrap().val {
                let next = p2.as_mut().unwrap().next.take();
                p.next = p2;
                p = p.next.as_mut().unwrap();
                p2 = next;
            } else {
                let next = p1.as_mut().unwrap().next.take();
                p.next = p1;
                p = p.next.as_mut().unwrap();
                p1 = next;
            }
        }

        if p1.is_some() {
            p.next = p1;
        }

        if p2.is_some() {
            p.next = p2;
        }

        dummy.next
    }
}

fn main() {
    // your test code here
    let l1 = ListNode::from_vec(vec![1, 3, 5]);
    let l2 = ListNode::from_vec(vec![2, 4, 6]);

    let solution = Solution::new();
    let result = solution.merge_two_lists(l1, l2);
    ListNode::print(result);
}