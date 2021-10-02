// 174. Dungeon Game, Hard
// https://leetcode.com/problems/dungeon-game/
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let [n, m] = [dungeon.len(), dungeon[0].len()];
        let mut dp = vec![vec![i32::max_value(); m + 1]; n + 1]; // (curr_health, min_health)

        dp[n - 1][m] = 1;
        dp[n][m - 1] = 1;

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                dp[i][j] = i32::max(i32::min(dp[i + 1][j], dp[i][j + 1]) - dungeon[i][j], 1);
            }
        }

        dp[0][0]
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_calculate_minimum_hp() {
        assert_eq!(Solution::calculate_minimum_hp(vec_vec_i32![[-2, -3, 3], [-5, -10, 1], [10, 30, -5]]), 7);
    }

    #[test]
    fn test_calculate_minimum_hp2() {
        assert_eq!(Solution::calculate_minimum_hp(vec_vec_i32![[0]]), 1);
    }

    #[test]
    fn test_calculate_minimum_hp3() {
        assert_eq!(Solution::calculate_minimum_hp(vec_vec_i32![[100]]), 1);
    }

    #[test]
    fn test_calculate_minimum_hp4() {
        assert_eq!(Solution::calculate_minimum_hp(vec_vec_i32![[1, -3, 3], [0, -2, 0], [-3, -3, -3]]), 3);
    }
}
