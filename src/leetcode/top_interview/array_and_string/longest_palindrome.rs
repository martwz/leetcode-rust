// 5. Longest Palindromic Substring, Medium
// https://leetcode.com/problems/longest-palindromic-substring/
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        if n <= 1 {
            return s;
        }

        let chars = s.chars().collect::<Vec<char>>();

        let mut ans = chars[0].to_string();
        let mut dp = vec![vec![false; n]; n];
        for i in 0..n {
            dp[i][i] = true;
        }

        let mut max_len = 0;
        for end in 1..n {
            for start in 0..end {
                if chars[start] == chars[end] && (end - start == 1 || dp[start + 1][end - 1]) {
                    dp[start][end] = true;
                    if max_len < end - start + 1 {
                        max_len = end - start + 1;
                        ans = chars[start..=end].iter().collect::<String>();
                    }
                }
            }
        }

        ans
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_i32, vec_vec_string};

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
    }

    #[test]
    fn test_longest_palindrome2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
    }

    #[test]
    fn test_longest_palindrome3() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a".to_string());
    }

    #[test]
    fn test_longest_palindrome4() {
        assert_eq!(Solution::longest_palindrome("ac".to_string()), "a".to_string());
    }
}
