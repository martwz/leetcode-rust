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
use std::collections::HashMap;
use std::rc::Rc;

// 1161. Maximum Level Sum of a Binary Tree, Medium
// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut level_sum = HashMap::<i32, i32>::new();

        fn bfs(node: Option<Rc<RefCell<TreeNode>>>, level: i32, level_sum: &mut HashMap<i32, i32>) {
            if node.is_some() {
                let node = node.as_ref().unwrap().borrow();

                level_sum.entry(level).and_modify(|e| *e += node.val).or_insert(node.val);

                bfs(node.left.clone(), level + 1, level_sum);
                bfs(node.right.clone(), level + 1, level_sum);
            }
        }

        bfs(root, 1, &mut level_sum);

        let max_value = *level_sum.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().1;
        *level_sum.iter().filter(|x| x.1 == &max_value).map(|x| x.0).min().unwrap()
    }
}

struct Solution {}
