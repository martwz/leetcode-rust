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

// 1008. Construct Binary Search Tree from Preorder Traversal, Medium
// https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/
impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(i: &mut usize, preorder: &Vec<i32>, lower: i32, upper: i32) -> Option<Rc<RefCell<TreeNode>>> {
            let n = preorder.len();

            if *i >= n {
                return None;
            }

            let val = preorder[*i];
            if val < lower || val > upper {
                return None;
            }

            *i += 1;

            let root = Rc::new(RefCell::new(TreeNode::new(val)));
            root.borrow_mut().left = helper(i, &preorder, lower, val);
            root.borrow_mut().right = helper(i, &preorder, val, upper);

            return Some(root);
        }

        let mut i = 0;
        return helper(&mut i, &preorder, i32::MIN, i32::MAX);
    }
}

struct Solution {}
