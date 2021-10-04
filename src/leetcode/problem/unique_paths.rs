// 62. Unique Paths, Medium
// https://leetcode.com/problems/unique-paths/
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let [n, m] = [n as usize, m as usize];
        let mut dp = vec![vec![0; m]; n];

        for i in 0..n {
            for j in 0..m {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }

        return dp[n - 1][m - 1];
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn test_unique_paths2() {
        assert_eq!(Solution::unique_paths(2, 3), 3);
    }

    #[test]
    fn test_unique_paths3() {
        assert_eq!(Solution::unique_paths(7, 3), 28);
    }

    #[test]
    fn test_unique_paths4() {
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }
}
