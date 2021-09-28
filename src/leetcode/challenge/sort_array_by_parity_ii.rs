use std::collections::VecDeque;

// 922. Sort Array By Parity II, Easy
// https://leetcode.com/problems/sort-array-by-parity-ii/
impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans: Vec<i32> = Vec::new();

        let mut p1: VecDeque<i32> = VecDeque::new();
        let mut p2: VecDeque<i32> = VecDeque::new();

        for num in nums {
            if num % 2 == 0 {
                p1.push_back(num);
            } else {
                p2.push_back(num);
            }
        }

        for i in 0..n {
            if i % 2 == 0 {
                ans.push(p1.pop_front().unwrap());
            } else {
                ans.push(p2.pop_front().unwrap());
            }
        }

        return ans;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array_by_parity_ii() {
        assert_eq!(Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]).len(), 4);
    }

    #[test]
    fn test_sort_array_by_parity_ii2() {
        assert_eq!(Solution::sort_array_by_parity_ii(vec![2, 3]), vec![2, 3]);
    }
}
