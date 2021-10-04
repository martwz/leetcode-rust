// 200. Number of Islands, Medium
// https://leetcode.com/problems/number-of-islands/
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let [n, m] = [grid.len(), grid[0].len()];

        fn bfs(grid: &mut Vec<Vec<char>>, pos: (i32, i32)) {
            grid[pos.0 as usize][pos.1 as usize] = '2';

            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            for direction in directions.iter() {
                let next_pos = (pos.0 + direction.0, pos.1 + direction.1);

                if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= grid.len() as i32 || next_pos.1 >= grid[0].len() as i32 {
                    continue;
                }

                if grid[next_pos.0 as usize][next_pos.1 as usize] == '1' {
                    bfs(grid, next_pos);
                }
            }
        }

        let mut islands = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '1' {
                    islands += 1;
                    bfs(&mut grid, (i as i32, j as i32));
                }
            }
        }

        islands
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_char};

    #[test]
    fn test_num_islands() {
        assert_eq!(
            Solution::num_islands(vec_vec_char![
                ['1', '1', '1', '1', '0'],
                ['1', '1', '0', '1', '0'],
                ['1', '1', '0', '0', '0'],
                ['0', '0', '0', '0', '0']
            ]),
            1
        );
    }

    #[test]
    fn test_num_islands2() {
        assert_eq!(
            Solution::num_islands(vec_vec_char![
                ['1', '1', '0', '0', '0'],
                ['1', '1', '0', '0', '0'],
                ['0', '0', '1', '0', '0'],
                ['0', '0', '0', '1', '1']
            ]),
            3
        );
    }

    #[test]
    fn test_num_islands3() {
        assert_eq!(Solution::num_islands(vec_vec_char![['1'], ['1']]), 1);
    }
}
