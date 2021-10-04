use std::collections::VecDeque;

// 55. Jump Game, Medium
// https://leetcode.com/problems/jump-game/
impl Solution {
    pub fn can_jump(mut nums: Vec<i32>) -> bool {
        let n = nums.len() - 1;
        let mut pos: VecDeque<usize> = VecDeque::new();
        pos.push_back(0);

        while pos.len() > 0 && pos.back().unwrap() < &n {
            let curr = *pos.back().unwrap();
            let next = nums[curr] as usize;

            if next == 0 {
                &pos.pop_back();
            } else {
                nums[curr] -= 1;
                pos.push_back(curr + next);
            }
        }

        pos.len() > 0 && pos.back().unwrap() >= &n
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_i32, vec_vec_string};

    #[test]
    fn test_can_jump() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn test_can_jump2() {
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn test_can_jump3() {
        assert_eq!(Solution::can_jump(vec![3, 0, 8, 2, 0, 0, 1]), true);
    }

    #[test]
    fn test_can_jump4() {
        assert_eq!(Solution::can_jump(vec![0]), true);
    }
}
