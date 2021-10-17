// 1861. Rotating the Box, Medium
// https://leetcode.com/problems/rotating-the-box/
impl Solution {
    pub fn rotate_the_box(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let [n, m] = [grid.len(), grid[0].len()];

        // gravity -> move everything to the right
        let mut grid = grid;
        for i in 0..n {
            for j in (0..m).rev() {
                if grid[i][j] == '.' {
                    let mut k = j;
                    let mut stone_k = -1;
                    while k > 0 && grid[i][k] != '*' {
                        k -= 1;
                        if grid[i][k] == '#' {
                            stone_k = k as i32;
                        }
                    }
                    if stone_k != -1 {
                        grid[i][stone_k as usize] = '.';
                        grid[i][j] = '#';
                    }
                }
            }
        }

        let mut res = vec![vec!['.'; n]; m];

        for i in 0..n {
            for j in 0..m {
                res[j][n - i - 1] = grid[i][j];
            }
        }

        res
    }
}

struct Solution {}
