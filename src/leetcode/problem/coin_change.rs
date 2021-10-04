// 322. Coin Change, Medium
// https://leetcode.com/problems/coin-change/
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let _n = coins.len();
        let mut dp = vec![amount + 1; amount as usize + 1];

        dp[0] = 0;

        for i in 1..=amount {
            for coin in coins.iter() {
                if i >= *coin {
                    dp[i as usize] = dp[i as usize].min(dp[(i - *coin) as usize] + 1);
                }
            }
        }

        if dp.last().unwrap() > &amount {
            -1
        } else {
            *dp.last().unwrap() as i32
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_coin_change() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn test_coin_change2() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn test_coin_change3() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }

    #[test]
    fn test_coin_change4() {
        assert_eq!(Solution::coin_change(vec![1], 1), 1);
    }

    #[test]
    fn test_coin_change5() {
        assert_eq!(Solution::coin_change(vec![1], 2), 2);
    }
}
