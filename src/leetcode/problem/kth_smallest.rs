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

// 230. Kth Smallest Element in a BST, Medium
// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn bfs(node: Option<Rc<RefCell<TreeNode>>>, k: i32, c: &mut i32, target: &mut i32) {
            if let Some(node) = node {
                bfs(node.borrow().left.clone(), k, c, target);
                *c += 1;
                if *c == k {
                    *target = node.borrow().val;
                    return;
                }
                bfs(node.borrow().right.clone(), k, c, target);
            }
        }

        let mut c = 0;
        let mut target = -1;
        bfs(root, k, &mut c, &mut target);

        target
    }
}

struct Solution {}
