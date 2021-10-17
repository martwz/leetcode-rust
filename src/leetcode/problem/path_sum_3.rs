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

// 437. Path Sum III, Medium
// https://leetcode.com/problems/path-sum-iii/
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn bfs(node: Option<Rc<RefCell<TreeNode>>>, sums: Vec<i32>, target_sum: i32, res: &mut i32) {
            if let Some(node) = node {
                let node = node.borrow_mut();
                let mut new_sums = Vec::new();
                new_sums.push(node.val);
                for sum in sums {
                    new_sums.push(sum + node.val);
                    if sum + node.val == target_sum {
                        *res += 1;
                    }
                }
                if node.val == target_sum {
                    *res += 1;
                }

                bfs(node.left.clone(), new_sums.clone(), target_sum, res);
                bfs(node.right.clone(), new_sums.clone(), target_sum, res);
            }
        }

        let mut res = 0;
        bfs(root, vec![], target_sum, &mut res);
        res
    }
}

struct Solution {}
