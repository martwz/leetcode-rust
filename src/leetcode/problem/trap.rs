use std::collections::VecDeque;

// 42. Trapping Rain Water, Hard
// https://leetcode.com/problems/trapping-rain-water/
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut q = VecDeque::<usize>::new();
        
        for (i, h) in height.iter().enumerate() {
            while q.len() > 0 && height[*q.back().unwrap()] < *h {
                let top = q.pop_back().unwrap();
                if q.len() == 0 {
                    break;
                }

                let dist = i - q.back().unwrap() - 1;
                let bounded_height = i32::min(*h, height[*q.back().unwrap()]) - height[top];
                ans += dist as i32 * bounded_height;
            }

            q.push_back(i);
        }

        ans
    }
}

struct Solution {}

mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_i32};

    #[test]
    fn test_trap() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn test_trap2() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
