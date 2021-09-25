// 1035. Uncrossed Lines, Medium
// https://leetcode.com/problems/uncrossed-lines/
impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let [n, m] = [nums1.len(), nums2.len()];
        let mut dp = vec![vec![0; m + 1]; n + 1];

        for i in 1..n + 1 {
            for j in 1..m + 1 {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = i32::min(dp[i - 1][j], i32::min(dp[i][j - 1], dp[i - 1][j - 1])) + 1
                } else {
                    dp[i][j] = i32::max(dp[i - 1][j], i32::max(dp[i][j - 1], dp[i - 1][j - 1]));
                }
            }
        }

        dp[n][m]
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_max_uncrossed_lines() {
        assert_eq!(Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]), 2);
    }

    #[test]
    fn test_max_uncrossed_lines2() {
        assert_eq!(Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]), 3);
    }

    #[test]
    fn test_max_uncrossed_lines3() {
        assert_eq!(Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]), 2);
    }
}
