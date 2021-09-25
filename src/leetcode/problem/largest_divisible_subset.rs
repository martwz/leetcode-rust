use std::collections::HashMap;

// 368. Largest Divisible Subset, Medium
// https://leetcode.com/problems/largest-divisible-subset/
impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        nums.sort_unstable();

        let mut sol = vec![];
        for i in 0..nums.len() {
            sol.push(vec![nums[i]]);
        }

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && sol[i].len() < sol[j].len() + 1 {
                    let mut append = sol[j].clone();
                    append.push(nums[i]);
                    sol[i] = append;
                }
            }
        }

        sol.sort_by_key(|a| a.len());

        return sol.last().unwrap().to_vec();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_divisible_subset() {
        assert_eq!(Solution::largest_divisible_subset(vec![1, 2, 3]), vec![1, 3]);
    }

    #[test]
    fn test_largest_divisible_subset2() {
        assert_eq!(Solution::largest_divisible_subset(vec![1, 2, 4, 8]), vec![1, 2, 4, 8]);
    }
}
