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
        fn bfs(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32, parent: i32, grandparent: i32) {
            if node.is_none() {
                return;
            }

            let node = node.as_ref().unwrap().borrow();

            if grandparent != -1 && grandparent % 2 == 0 {
                *sum += node.val;
            }

            bfs(node.left.clone(), sum, node.val, parent);
            bfs(node.right.clone(), sum, node.val, parent);
        }

        let mut sum = 0;
        bfs(root, &mut sum, -1, -1);

        sum
    }
}

struct Solution {}
