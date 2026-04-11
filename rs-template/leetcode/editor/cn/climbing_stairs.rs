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
        // 只需保留前两个状态，空间复杂度 O(1)
        let mut prev2 = 1i32; // dp[i-2]
        let mut prev1 = 2i32; // dp[i-1]
        for _ in 3..=n {
            let cur = prev1 + prev2;
            prev2 = prev1;
            prev1 = cur;
        }
        prev1
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
