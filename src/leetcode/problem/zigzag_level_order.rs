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

// 103. Binary Tree Zigzag Level Order Traversal, Medium
// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        fn bfs(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<Vec<i32>>, level: usize) {
            if let Some(node) = node {
                if ans.len() <= level {
                    ans.push(vec![]);
                }

                if level % 2 == 0 {
                    ans[level].push(node.borrow().val);
                } else {
                    ans[level].insert(0, node.borrow().val);
                }

                bfs(node.borrow().left.clone(), ans, level + 1);
                bfs(node.borrow().right.clone(), ans, level + 1);
            }
        }

        bfs(root, &mut ans, 0);

        ans
    }
}

struct Solution {}
