/*
 * @lc app=leetcode.cn id=2415 lang=rust
 *
 * [2415] 反转二叉树的奇数层
 *
 * https://leetcode.cn/problems/reverse-odd-levels-of-binary-tree/description/
 *
 */

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

// @lc code=start
// Definition for a binary tree node.
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }
        let root = root.unwrap().clone();
        let mut deque = VecDeque::new();
        deque.push_back(root.clone());

        let mut odd_nodes = vec![];
        odd_nodes.push(root.clone().borrow().val);

        let mut odd = false;

        while !deque.is_empty() {
            let mut next = VecDeque::new();

            while let Some(node) = deque.pop_front() {
                let mut node = node.borrow_mut();
                if let Some(left) = node.left.clone() {
                    next.push_back(left);
                }
                if let Some(right) = node.right.clone() {
                    next.push_back(right);
                }
                if odd {
                    node.val = odd_nodes.pop().unwrap();
                }
            }
            if !odd {
                odd_nodes = next
                    .iter()
                    .map(|node| node.borrow().val)
                    .collect();
            }
            deque = next;
            odd = !odd;
        }

        return Some(root);
    }
}
// @lc code=end

#[cfg(test)]
mod tests {}
