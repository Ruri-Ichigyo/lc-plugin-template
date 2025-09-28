/*
 * @lc app=leetcode.cn id=117 lang=rust
 * @lcpr version=30203
 *
 * [117] 填充每个节点的下一个右侧节点指针 II
 */

// @lc code=start
// Definition for a Node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct Node {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<Node>>>,
//   pub right: Option<Rc<RefCell<Node>>>,
//   pub next: Option<Rc<RefCell<Node>>>,
// }
//
// impl Node {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     Node {
//       val,
//       left: None,
//       right: None,
//       next: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn connect(root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut leftmost = root.clone();
        while let Some(level_start) = leftmost {
            let mut head = Some(level_start.clone());
            let mut dummy = Rc::new(RefCell::new(Node::new(0)));
            let mut prev = dummy.clone();
            while let Some(node) = head {
                let node_ref = node.borrow();
                if let Some(left) = node_ref.left.as_ref() {
                    prev.borrow_mut().next = Some(left.clone());
                    prev = left.clone();
                }
                if let Some(right) = node_ref.right.as_ref() {
                    prev.borrow_mut().next = Some(right.clone());
                    prev = right.clone();
                }
                head = node_ref.next.clone();
            }
            leftmost = dummy.borrow().next.clone();
        }
        root
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,3,4,5,null,7]\n
// @lcpr case=end

// @lcpr case=start
// []\n
// @lcpr case=end

 */
