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
        queue.push_back(root.as_ref().unwrap().as_ref() as *const TreeNode as *mut TreeNode);

        let mut i = 1;
        while i < values.len() {
            let current = queue.pop_front().unwrap();

            // Process left child
            if i < values.len() {
                if let Some(val) = values[i] {
                    unsafe {
                        (*current).left = Some(Box::new(TreeNode::new(val)));
                        queue.push_back((*current).left.as_ref().unwrap().as_ref() as *const TreeNode as *mut TreeNode);
                    }
                }
                i += 1;
            }

            // Process right child
            if i < values.len() {
                if let Some(val) = values[i] {
                    unsafe {
                        (*current).right = Some(Box::new(TreeNode::new(val)));
                        queue.push_back((*current).right.as_ref().unwrap().as_ref() as *const TreeNode as *mut TreeNode);
                    }
                }
                i += 1;
            }
        }

        root
    }

    pub fn print(root: Option<Box<TreeNode>>) {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
        }
        while let Some(node) = queue.pop_front() {
            result.push(Some(node.val));
            if let Some(left) = node.left {
                queue.push_back(left);
            } else if node.right.is_some() {
                result.push(None);
            }
            if let Some(right) = node.right {
                queue.push_back(right);
            }
        }
        let strs: Vec<String> = result
            .iter()
            .map(|v| match v {
                Some(n) => n.to_string(),
                None => "null".to_string(),
            })
            .collect();
        println!("[{}]", strs.join(","));
    }
}