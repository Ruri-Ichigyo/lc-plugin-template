/*
 * @lc app=leetcode.cn id=3 lang=rust
 * @lcpr version=30203
 *
 * [3] 无重复字符的最长子串
 */

use std::collections::HashMap;

// @lc code=start
struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // 滑动窗口：记录窗口内字符最近一次出现的位置
        let mut map: HashMap<char, usize> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        let mut left = 0usize;
        let mut max_len = 0usize;

        for right in 0..chars.len() {
            if let Some(&prev) = map.get(&chars[right]) {
                // 若字符已在窗口内，收缩左边界
                if prev >= left {
                    left = prev + 1;
                }
            }
            map.insert(chars[right], right);
            max_len = max_len.max(right - left + 1);
        }
        max_len as i32
    }
}
// @lc code=end

fn main() {
    // your test code here
    println!("{}", Solution::length_of_longest_substring("abcabcbb".to_string())); // 3
    println!("{}", Solution::length_of_longest_substring("bbbbb".to_string()));    // 1
    println!("{}", Solution::length_of_longest_substring("pwwkew".to_string()));   // 3
}

/*
// @lcpr case=start
// "abcabcbb"\n
// @lcpr case=end

// @lcpr case=start
// "bbbbb"\n
// @lcpr case=end

// @lcpr case=start
// "pwwkew"\n
// @lcpr case=end

 */
