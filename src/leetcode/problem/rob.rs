// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

// 337. House Robber III, Medium
// https://leetcode.com/problems/house-robber-iii/
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn bfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if node.is_some() {
                let node = node.as_ref().unwrap().borrow();

                let l = bfs(node.left.clone());
                let r = bfs(node.left.clone());

                (node.val + l.1 + r.1, i32::max(l.0, l.1) + i32::max(r.0, r.1))
            } else {
                (0, 0)
            }
        }

        i32::max(bfs(root.clone()).0, bfs(root.clone()).1)
    }
}

struct Solution {}
