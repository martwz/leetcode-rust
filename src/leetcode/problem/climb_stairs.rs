// 70. Climbing Stairs, Easy
// https://leetcode.com/problems/climbing-stairs/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; n.max(4) as usize + 1];
            dp[0] = 0;
            dp[1] = 1;
            dp[2] = 2;
            dp[3] = 3;
            for i in 4..=n as usize {
                dp[i] = dp[i - 1] + dp[i - 2];
            }
            dp[n as usize]
        }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test_climb_stairs2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn test_climb_stairs3() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }
}

