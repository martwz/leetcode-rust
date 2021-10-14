// 279. Perfect Squares, Medium
// https://leetcode.com/problems/perfect-squares/
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        for i in 1..=n as usize {
            let mut min = i as i32;
            let mut j = 1;
            while j * j <= i {
                min = std::cmp::min(min, dp[i - j * j] + 1);
                j += 1;
            }
            dp[i] = min;
        }
        dp[n as usize]
    }
}

struct Solution {}
