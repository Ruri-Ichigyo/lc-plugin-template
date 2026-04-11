/*
 * @lc app=leetcode.cn id=104 lang=rust
 * @lcpr version=30203
 *
 * [104] 二叉树的最大深度
 */

use leetcode::TreeNode;

// @lc code=start
struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left_depth = Self::max_depth(node.left);
                let right_depth = Self::max_depth(node.right);
                1 + left_depth.max(right_depth)
            }
        }
    }
}
// @lc code=end

fn main() {
    // your test code here
    let root = TreeNode::from_vec(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    println!("{}", Solution::max_depth(root)); // 3

    let root2 = TreeNode::from_vec(vec![Some(1), None, Some(2)]);
    println!("{}", Solution::max_depth(root2)); // 2
}

/*
// @lcpr case=start
// [3,9,20,null,null,15,7]\n
// @lcpr case=end

// @lcpr case=start
// [1,null,2]\n
// @lcpr case=end

 */
