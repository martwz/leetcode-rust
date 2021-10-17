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

// 100. Same Tree, Easy
// https://leetcode.com/problems/same-tree/
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn bfs(node1: Option<Rc<RefCell<TreeNode>>>, node2: Option<Rc<RefCell<TreeNode>>>) -> bool {
            if let (Some(node1), Some(node2)) = (node1.clone(), node2.clone()) {
                if node1.borrow().val != node2.borrow().val {
                    return false;
                }

                return bfs(node1.borrow().left.clone(), node2.borrow().left.clone()) && bfs(node1.borrow().right.clone(), node2.borrow().right.clone());
            } else { node1.is_none() && node2.is_none() }
        }

        bfs(p, q)
    }
}

struct Solution {}
