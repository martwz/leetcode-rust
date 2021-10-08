// 516. Longest Palindromic Subsequence, Medium
// https://leetcode.com/problems/longest-palindromic-subsequence/
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];

        let chars = s.chars().collect::<Vec<char>>();

        for i in 0..n {
            dp[i][i] = 1;
        }

        for i in 1..n {
            for j in 0..s.len() - i {
                let i = j + i;
                if chars[j] == chars[i] {
                    dp[j][i] = 2 + dp[j + 1][i - 1];
                } else {
                    dp[j][i] = i32::max(dp[j + 1][i], dp[j][i - 1]);
                }
            }
        }

        dp[0][n - 1]
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_vec_char, vec_vec_i32};

    #[test]
    fn test_longest_palindrome_subseq() {
        assert_eq!(Solution::longest_palindrome_subseq("bbbab".to_string()), 4);
    }

    #[test]
    fn test_longest_palindrome_subseq2() {
        assert_eq!(Solution::longest_palindrome_subseq("cbbd".to_string()), 2);
    }

    #[test]
    fn test_longest_palindrome_subseq3() {
        assert_eq!(Solution::longest_palindrome_subseq("cbbd".to_string()), 2);
    }
}
