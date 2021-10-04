// 463. Island Perimeter, Easy
// https://leetcode.com/problems/island-perimeter/
impl Solution {
    pub fn island_perimeter(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut size = 0;

        fn bfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, size: &mut i32) {
            if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 || grid[i as usize][j as usize] == 0 {
                *size += 1;
                return;
            } else if grid[i as usize][j as usize] == 2 {
                return;
            }

            grid[i as usize][j as usize] = 2;

            bfs(grid, i + 1, j, size);
            bfs(grid, i, j + 1, size);
            bfs(grid, i - 1, j, size);
            bfs(grid, i, j - 1, size);
        }

        'outer: for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    bfs(&mut grid, i as i32, j as i32, &mut size);
                    break 'outer;
                }
            }
        }

        size
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_island_perimeter() {
        assert_eq!(Solution::island_perimeter(vec_vec_i32![[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [1, 1, 0, 0]]), 16);
    }

    #[test]
    fn test_island_perimeter2() {
        assert_eq!(Solution::island_perimeter(vec_vec_i32![[1]]), 4);
    }

    #[test]
    fn test_island_perimeter3() {
        assert_eq!(Solution::island_perimeter(vec_vec_i32![[1, 0]]), 4);
    }
}
