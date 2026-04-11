/*
 * @lc app=leetcode.cn id=20 lang=rust
 * @lcpr version=30203
 *
 * [20] 有效的括号
 */

// @lc code=start
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => {}
            }
        }
        stack.is_empty()
    }
}
// @lc code=end

fn main() {
    // your test code here
    println!("{}", Solution::is_valid("()".to_string()));       // true
    println!("{}", Solution::is_valid("()[]{}".to_string()));   // true
    println!("{}", Solution::is_valid("(]".to_string()));       // false
}

/*
// @lcpr case=start
// "()"\n
// @lcpr case=end

// @lcpr case=start
// "()[]{}"\n
// @lcpr case=end

// @lcpr case=start
// "(]"\n
// @lcpr case=end

 */
