/*
 * @lc app=leetcode.cn id=70 lang=rust
 * @lcpr version=30203
 *
 * [70] 爬楼梯
 */

// @lc code=start
struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        // dp[i] 表示爬到第 i 阶楼梯的方法数
        let mut dp = vec![0i32; (n + 1) as usize];
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=(n as usize) {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }
}
// @lc code=end

fn main() {
    // your test code here
    println!("{}", Solution::climb_stairs(2)); // 2
    println!("{}", Solution::climb_stairs(3)); // 3
    println!("{}", Solution::climb_stairs(10)); // 89
}

/*
// @lcpr case=start
// 2\n
// @lcpr case=end

// @lcpr case=start
// 3\n
// @lcpr case=end

 */
