use std::collections::BinaryHeap;

// 1962. Remove Stones to Minimize the Total, Medium
// https://leetcode.com/problems/remove-stones-to-minimize-the-total/
impl Solution {
    pub fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(piles);

        for _ in 0..k {
            let mut t = heap.pop().unwrap_or(0);
            t -= f32::floor(t as f32 / 2.0) as i32;
            heap.push(t);
        }

        heap.iter().sum()
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_min_stone_sum() {
        assert_eq!(Solution::min_stone_sum(vec![5, 4, 9], 2), 12);
    }

    #[test]
    fn test_min_stone_sum2() {
        assert_eq!(Solution::min_stone_sum(vec![4, 3, 6, 7], 3), 12);
    }
}
