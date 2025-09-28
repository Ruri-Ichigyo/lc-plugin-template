use leetcode::TreeNode;

struct Solution {
    res: Vec<i32>,
}

impl Solution {
    fn new() -> Self {
        Solution {
            res: Vec::new(),
        }
    }

    // Backtracking approach
    // Return the preorder traversal result
    fn preorder_traversal(&mut self, root: Option<Box<TreeNode>>) -> Vec<i32> {
        self.traverse(root.as_ref());
        self.res.clone()
    }

    // Binary tree traversal function
    fn traverse(&mut self, root: Option<&Box<TreeNode>>) {
        if let Some(node) = root {
            // Preorder traversal position
            self.res.push(node.val);
            self.traverse(node.left.as_ref());
            self.traverse(node.right.as_ref());
        }
    }
}

fn main() {
    let tree_data = vec![
        Some(1),
        Some(22),
        Some(3),
        Some(4),
        Some(5),
        None,
        Some(8),
        None,
        None,
        Some(6),
        Some(7),
        Some(9),
    ];

    let root = TreeNode::from_vec(tree_data);
    TreeNode::print(root.clone());

    let mut solution = Solution::new();
    let result = solution.preorder_traversal(root);
    for val in result {
        print!("{} ", val);
    }
    println!();
}