/*
 * @lc app=leetcode.cn id=704 lang=rust
 * @lcpr version=30203
 *
 * [704] 二分查找
 */

// @lc code=start
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0i32;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return mid;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        -1
    }
}
// @lc code=end

fn main() {
    // your test code here
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;
    let result = Solution::search(nums, target);
    println!("{}", result); // 4

    let nums2 = vec![-1, 0, 3, 5, 9, 12];
    let target2 = 2;
    println!("{}", Solution::search(nums2, target2)); // -1
}

/*
// @lcpr case=start
// [-1,0,3,5,9,12]\n9\n
// @lcpr case=end

// @lcpr case=start
// [-1,0,3,5,9,12]\n2\n
// @lcpr case=end

 */
