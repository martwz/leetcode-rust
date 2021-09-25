// 115. Distinct Subsequences, Hard
// https://leetcode.com/problems/distinct-subsequences/
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let [m, n] = [s.len(), t.len()];
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..m {
            dp[i][0] = 1;
        }

        for i in 1..m+1 {
            for j in 1..n+1 {
                if s.chars().nth(i - 1) == t.chars().nth(j - 1) {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }

        dp[m][n]
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_distinct() {
        assert_eq!(Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
    }

    #[test]
    fn test_num_distinct2() {
        assert_eq!(Solution::num_distinct("babgbag".to_string(), "bag".to_string()), 5);
    }
}
