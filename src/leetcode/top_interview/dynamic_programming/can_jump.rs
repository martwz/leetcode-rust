use std::collections::VecDeque;

// 55. Jump Game, Medium
// https://leetcode.com/problems/jump-game/
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut pos: VecDeque<usize> = VecDeque::new();
        pos.push_back(0);
        let mut jumps = nums.clone();

        while !pos.is_empty() && *pos.back().unwrap() < nums.len() - 1 {
            let mut p = *pos.back().unwrap();
            if jumps[p] == 0 && pos.len() > 1 {
                pos.pop_back();
                p = *pos.back().unwrap();
                jumps[p] -= 1;
            } else if jumps[p] == 0 && pos.len() <= 1 {
                return false;
            } else {
                p += jumps[p] as usize;
                pos.push_back(p);
            }
        }

        return !pos.is_empty() && *pos.back().unwrap() >= nums.len() - 1;
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
}
