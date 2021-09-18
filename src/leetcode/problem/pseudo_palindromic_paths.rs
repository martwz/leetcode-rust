use std::cell::RefCell;
use std::rc::Rc;

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

// https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/
impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;

        fn dfs(ans: &mut i32, node: Option<Rc<RefCell<TreeNode>>>, mut mask: i32) {
            if node.is_none() {
                return;
            }

            let node = node.as_ref().unwrap().borrow();

            mask = mask ^ 1 << node.val - 1;
            if node.left.is_none() && node.right.is_none() {
                if mask == mask & -mask {
                    *ans += 1;
                }
            } else {
                dfs(ans, node.left.clone(), mask.clone());
                dfs(ans, node.right.clone(), mask.clone());
            }
        }

        dfs(&mut ans, root, 0);

        return ans;
    }
}

struct Solution {}
