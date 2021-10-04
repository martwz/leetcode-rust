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
use std::collections::VecDeque;
use std::rc::Rc;

// 94. Binary Tree Inorder Traversal, Easy
// https://leetcode.com/problems/binary-tree-inorder-traversal/
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];

        fn bfs(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(node) = node {
                bfs(node.borrow().left.clone(), ans);
                ans.push(node.borrow().val);
                bfs(node.borrow().right.clone(), ans);
            }
        }

        bfs(root, &mut ans);

        ans
    }
}

struct Solution {}
