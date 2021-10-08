use std::collections::BinaryHeap;

// 1167. Minimum Cost to Connect Sticks, Medium
// https://leetcode.com/problems/minimum-cost-to-connect-sticks/
impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        let sticks = sticks.iter().map(|x| -x).collect::<Vec<i32>>();
        let mut sticks_heap = BinaryHeap::from(sticks);

        let mut cost = 0;
        while sticks_heap.len() > 1 {
            let stick = sticks_heap.pop().unwrap() + sticks_heap.pop().unwrap();
            sticks_heap.push(stick);
            cost += -stick;
        }

        cost
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_connect_sticks() {
        assert_eq!(Solution::connect_sticks(vec![2, 4, 3]), 14);
    }

    #[test]
    fn test_connect_sticks2() {
        assert_eq!(Solution::connect_sticks(vec![1, 8, 3, 5]), 30);
    }
}
