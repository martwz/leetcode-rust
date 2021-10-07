use std::collections::HashMap;

// 930. Binary Subarrays With Sum, Medium
// https://leetcode.com/problems/binary-subarrays-with-sum/
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut ans = 0;
        let mut prefix_sum = 0;
        let mut prefix_sum_map = HashMap::new();
        for num in nums {
            prefix_sum += num;
            if prefix_sum == goal {
                ans += 1;
            }
            if let Some(count) = prefix_sum_map.get(&(prefix_sum - goal)) {
                ans += *count;
            }

            prefix_sum_map.entry(prefix_sum).and_modify(|e| *e += 1).or_insert(1);
        }

        ans
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
    }

    #[test]
    fn test_find_duplicates2() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0), 15);
    }
}
