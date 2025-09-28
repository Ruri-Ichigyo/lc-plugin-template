use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
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

    pub fn from_vec(values: Vec<Option<i32>>) -> Option<Box<TreeNode>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Some(Box::new(TreeNode::new(values[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.as_ref().unwrap() as *const TreeNode as *mut TreeNode);

        let mut i = 1;
        while i < values.len() {
            let current = queue.pop_front().unwrap();

            // Process left child
            if i < values.len() {
                if let Some(val) = values[i] {
                    unsafe {
                        (*current).left = Some(Box::new(TreeNode::new(val)));
                        queue.push_back((*current).left.as_ref().unwrap() as *const TreeNode as *mut TreeNode);
                    }
                }
                i += 1;
            }

            // Process right child
            if i < values.len() {
                if let Some(val) = values[i] {
                    unsafe {
                        (*current).right = Some(Box::new(TreeNode::new(val)));
                        queue.push_back((*current).right.as_ref().unwrap() as *const TreeNode as *mut TreeNode);
                    }
                }
                i += 1;
            }
        }

        root
    }
}