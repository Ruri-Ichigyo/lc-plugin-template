/*
 * @lc app=leetcode.cn id=155 lang=rust
 * @lcpr version=30203
 *
 * [155] 最小栈
 */

// @lc code=start
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>, // 辅助栈，栈顶始终是当前最小值
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        let min = match self.min_stack.last() {
            Some(&cur_min) => cur_min.min(val),
            None => val,
        };
        self.min_stack.push(min);
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}
// @lc code=end

fn main() {
    // your test code here
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    println!("{}", min_stack.get_min()); // -3
    min_stack.pop();
    println!("{}", min_stack.top());     // 0
    println!("{}", min_stack.get_min()); // -2
}

/*
// @lcpr case=start
// ["MinStack","push","push","push","getMin","pop","top","getMin"]\n[[],[-2],[0],[-3],[],[],[],[]]\n
// @lcpr case=end

 */
