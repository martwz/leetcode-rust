// 764. Largest Plus Sign, Medium
// https://leetcode.com/problems/largest-plus-sign/
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut grid = vec![vec![0; n]; n];

        for mine in mines.iter() {
            grid[mine[0] as usize][mine[1] as usize] = 1;
        }

        let mut prefix_sum_grid = vec![vec![i32::MAX; n]; n];

        for i in 0..n {
            let mut sum_right = 0;
            let mut sum_left = 0;
            let mut sum_down = 0;
            let mut sum_up = 0;

            for j in 0..n {
                sum_right += grid[i][j];
                prefix_sum_grid[i][j] += i32::min(prefix_sum_grid[i][j], sum_right);

                sum_left += grid[i][n - j - 1];
                prefix_sum_grid[i][n - j - 1] += i32::min(prefix_sum_grid[i][n - j - 1], sum_left);

                sum_down += grid[j][i];
                prefix_sum_grid[j][i] += i32::min(prefix_sum_grid[j][i], sum_down);

                sum_up += grid[n - j - 1][i];
                prefix_sum_grid[n - j - 1][i] += i32::min(prefix_sum_grid[n - j - 1][i], sum_up);
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for j in 0..n {
                ans = i32::max(ans, prefix_sum_grid[i][j]);
            }
        }

        ans
    }
}

struct Solution {}
