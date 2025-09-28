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

    // 返回前序遍历结果
    fn preorder_traversal(&mut self, root: Option<Box<TreeNode>>) -> Vec<i32> {
        self.traverse(root.as_ref());
        self.res.clone()
    }

    // 二叉树遍历函数
    fn traverse(&mut self, root: Option<&Box<TreeNode>>) {
        if let Some(node) = root {
            // 前序遍历位置
            self.res.push(node.val);
            self.traverse(node.left.as_ref());
            self.traverse(node.right.as_ref());
        }
    }
}

fn main() {
    let mut solution = Solution::new();
    
    // 构建测试用的二叉树: [1, 22, 3, 4, 5, null, 8, null, null, 6, 7, 9]
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
    let result = solution.preorder_traversal(root);
    
    for val in result {
        print!("{} ", val);
    }
    println!();
}