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

// 543. Diameter of Binary Tree, Easy
// https://leetcode.com/problems/diameter-of-binary-tree/
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn bfs(node: Option<Rc<RefCell<TreeNode>>>, max_path_length: &mut i32) -> i32 {
            if node.is_none() {
                return 0;
            }
            let node = node.as_ref().unwrap().borrow();

            let path_left = bfs(node.left.clone(), max_path_length);
            let path_right = bfs(node.right.clone(), max_path_length);

            let path_length = 1 + path_left + path_right;
            *max_path_length = i32::max(*max_path_length, path_length);

            1 + i32::max(path_left, path_right)
        }

        let mut max_path_length = 0;
        bfs(root, &mut max_path_length);

        max_path_length - 1
    }
}

struct Solution {}
