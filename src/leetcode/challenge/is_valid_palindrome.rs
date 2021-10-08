// 1216. Valid Palindrome III, Hard
// https://leetcode.com/problems/valid-palindrome-iii/
impl Solution {
    pub fn is_valid_palindrome(s: String, k: i32) -> bool {
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];

        let chars = s.chars().collect::<Vec<char>>();

        for i in 0..n {
            dp[i][i] = 1;
        }

        for i in 1..n {
            for j in 0..n - i {
                let i = j + i;
                if chars[j] == chars[i] {
                    dp[j][i] = 2 + dp[j + 1][i - 1];
                } else {
                    dp[j][i] = i32::max(dp[j + 1][i], dp[j][i - 1]);
                }
            }
        }

        dp[0][n - 1] >= n as i32 - k
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_vec_char, vec_vec_i32};

    #[test]
    fn test_is_valid_palindrome() {
        assert_eq!(Solution::is_valid_palindrome("abcdeca".to_string(), 2), true);
    }

    #[test]
    fn test_is_valid_palindrome2() {
        assert_eq!(Solution::is_valid_palindrome("abbababa".to_string(), 1), true);
    }
}
