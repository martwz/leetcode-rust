// 72. Edit Distance, Hard
// https://leetcode.com/problems/edit-distance/
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let [n, m] = [word1.len(), word2.len()];
        let mut dp = vec![vec![0; m + 1]; n + 1];

        for i in 0..n + 1 {
            dp[i][0] = i as i32;
        }
        for j in 0..m + 1 {
            dp[0][j] = j as i32;
        }

        for i in 1..n + 1 {
            for j in 1..m + 1 {
                if word1.chars().nth(i - 1) == word2.chars().nth(j - 1) {
                    dp[i][j] = dp[i - 1][j - 1]
                } else {
                    dp[i][j] = i32::min(dp[i - 1][j], i32::min(dp[i][j - 1], dp[i - 1][j - 1])) + 1;
                }
            }
        }

        return dp[n][m];
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_min_distance() {
        assert_eq!(Solution::min_distance("horse".to_string(), "ros".to_string()), 3);
    }

    #[test]
    fn test_min_distance2() {
        assert_eq!(Solution::min_distance("intention".to_string(), "execution".to_string()), 5);
    }

    #[test]
    fn test_min_distance3() {
        assert_eq!(Solution::min_distance("".to_string(), "a".to_string()), 1);
    }

    #[test]
    fn test_min_distance4() {
        assert_eq!(Solution::min_distance("b".to_string(), "".to_string()), 1);
    }
}
