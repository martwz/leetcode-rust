use std::collections::HashMap;

// 325. Maximum Size Subarray Sum Equals k, Medium
// https://leetcode.com/problems/maximum-size-subarray-sum-equals-k/solution/
impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let mut max_length = 0;

        let mut curr_sum = 0;
        let mut sum_to_index_mapping = HashMap::new();
        for i in 0..nums.len() {
            curr_sum += nums[i];

            if curr_sum == k {
                max_length = max_length.max(i + 1);
            } else if let Some(j) = sum_to_index_mapping.get(&(curr_sum - k)) {
                max_length = max_length.max(i - j);
            }

            if !sum_to_index_mapping.contains_key(&curr_sum) {
                sum_to_index_mapping.insert(curr_sum, i);
            }
        }

        return max_length as i32;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array_len() {
        assert_eq!(Solution::max_sub_array_len(vec![1, -1, 5, -2, 3], 3), 4);
    }

    #[test]
    fn test_max_sub_array_len2() {
        assert_eq!(Solution::max_sub_array_len(vec![-2, -1, 2, 1], 1), 2);
    }
}
