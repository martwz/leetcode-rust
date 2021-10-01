// 1143. Longest Common Subsequence, Medium
// https://leetcode.com/problems/longest-common-subsequence/
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let [n, m] = [text1.len(), text2.len()];
        let mut dp = vec![vec![0; n + 1]; m + 1];
        
        for i in 0..n {
            for j in 0..m {
                if text1.chars().nth(i).unwrap() == text2.chars().nth(j).unwrap() {
                    dp[j + 1][i + 1] = dp[j][i] + 1;
                } else {
                  dp[j + 1][i + 1] = dp[j + 1][i].max(dp[j][i + 1]);
                }
            }
        }

        *dp.last().unwrap().last().unwrap()
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence() {
        assert_eq!(Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()), 3);
    }

    #[test]
    fn test_longest_common_subsequence2() {
        assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()), 3);
    }

    #[test]
    fn test_longest_common_subsequence3() {
        assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
    }

    #[test]
    fn test_longest_common_subsequence4() {
        assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "".to_string()), 0);
    }
}
