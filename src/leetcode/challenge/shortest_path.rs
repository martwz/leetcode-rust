use std::collections::{HashSet};
use std::iter::FromIterator;

// 1293. Shortest Path in a Grid with Obstacles Elimination, Hard
// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        fn bfs(grid: &Vec<Vec<i32>>, pos: (i32, i32), obstacles_left: i32, mut visited: HashSet<(i32, i32)>, paths: &mut Vec<Vec<(i32, i32)>>) {
            let n = grid.len();
            if n == 0 {
                return;
            }
            let m = grid[0].len();

            // reached end?
            if pos.0 == n as i32 - 1 && pos.1 == m as i32 - 1 {
                paths.push(Vec::from_iter(visited));
                return;
            }

            visited.insert((pos.0, pos.1));

            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            for direction in directions {
                let next_pos = (pos.0 + direction.0, pos.1 + direction.1);
                let new_visited = visited.clone();
                // in bounds?
                if next_pos.0 < 0 || next_pos.0 >= n as i32 || next_pos.1 < 0 || next_pos.1 >= m as i32 {
                    continue;
                }
                // not visited?
                if new_visited.contains(&(next_pos.0, next_pos.1)) {
                    continue;
                }

                if grid[next_pos.0 as usize][next_pos.1 as usize] == 1 && obstacles_left > 0 {
                    bfs(grid, next_pos, obstacles_left - 1, new_visited, paths);
                } else if grid[next_pos.0 as usize][next_pos.1 as usize] == 0 {
                    bfs(grid, next_pos, obstacles_left, new_visited, paths);
                }
            }
        }

        let mut paths = Vec::new();

        // start is obstacle?
        if grid.len() > 0 && grid[0].len() > 0 && grid[0][0] == 1 {
            return -1;
        }
        bfs(&grid, (0, 0), k, HashSet::new(), &mut paths);

        paths.sort_by_key(|a| a.len());
        if let Some(path) = paths.first() {
            return path.len() as i32;
        } else {
            return -1;
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_shortest_path() {
        assert_eq!(Solution::shortest_path(vec_vec_i32![[0, 0, 0], [1, 1, 0], [0, 0, 0], [0, 1, 1], [0, 0, 0]], 1), 6);
    }

    #[test]
    fn test_shortest_path2() {
        assert_eq!(Solution::shortest_path(vec_vec_i32![[0, 1, 1], [1, 1, 1], [1, 0, 0]], 1), -1);
    }

    #[test]
    fn test_shortest_path3() {
        assert_eq!(Solution::shortest_path(vec_vec_i32![[]], 0), -1);
    }

    #[test]
    fn test_shortest_path4() {
        assert_eq!(Solution::shortest_path(vec_vec_i32![[1]], 0), -1);
    }
}
