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

// 993. Cousins in Binary Tree, Easy
// https://leetcode.com/problems/cousins-in-binary-tree/
impl Solution {
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut found: Vec<(i32, i32)> = vec![]; // (parent, level)

        fn bfs(node: Option<Rc<RefCell<TreeNode>>>, x: &i32, y: &i32, parent: i32, level: i32, found: &mut Vec<(i32, i32)>) {
            if node.is_some() {
                let node = node.as_ref().unwrap().borrow();

                if found.len() == 2 {
                    return;
                }

                if node.val == *x || node.val == *y {
                    found.push((parent, level));
                    return;
                }

                bfs(node.left.clone(), &x, &y, node.val, level + 1, found);
                bfs(node.right.clone(), &x, &y, node.val, level + 1, found);
            }
        }

        bfs(root, &x, &y, -1, 0, &mut found);

        found.len() == 2 && found[0].0 != found[1].0 && found[0].1 == found[1].1
    }
}

struct Solution {}
