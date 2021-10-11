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

// 1315. Sum of Nodes with Even-Valued Grandparent, Medium
// https://leetcode.com/problems/sum-of-nodes-with-even-valued-grandparent/
impl Solution {
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn bfs(node: Option<Rc<RefCell<TreeNode>>>, parent_val: i32, grand_parent_val: i32, ans: &mut i32) {
            if node.is_some() {
                let node = node.as_ref().unwrap().borrow();

                if grand_parent_val != -1 && grand_parent_val % 2 == 0 {
                    *ans += node.val;
                }

                bfs(node.left.clone(), node.val, parent_val, ans);
                bfs(node.left.clone(), node.val, parent_val, ans);
            }
        }

        let mut ans = 0;

        bfs(root, -1, -1, &mut ans);
        ans
    }
}

struct Solution {}
