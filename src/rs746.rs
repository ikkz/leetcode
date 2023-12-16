/*
 * @lc app=leetcode.cn id=746 lang=rust
 *
 * [746] 使用最小花费爬楼梯
 *
 * https://leetcode.cn/problems/min-cost-climbing-stairs/description/
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, 0);
        for k in 2..(cost.len() + 1) {
            let next = (i + cost[k - 2]).min(j + cost[k - 1]);
            i = j;
            j = next;
        }
        j
    }
}
// @lc code=end
