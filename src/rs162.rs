/*
 * @lc app=leetcode.cn id=162 lang=rust
 *
 * [162] 寻找峰值
 *
 * https://leetcode.cn/problems/find-peak-element/description/
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let el = |i: i32| {
            if i == -1 || i == nums.len() as i32 {
                i64::MIN
            } else {
                nums[i as usize] as i64
            }
        };
        let (mut left, mut right): (i32, i32) = (0, (nums.len() - 1) as i32);
        while left <= right {
            let mid = (left + right) / 2;
            if el(mid) > el(mid + 1) && el(mid) > el(mid - 1) {
                return mid;
            }
            if el(mid) < el(mid + 1) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        unreachable!()
    }
}
// @lc code=end
