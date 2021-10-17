// 695. Max Area of Island, Medium
// https://leetcode.com/problems/max-area-of-island/
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let [n, m] = [grid.len(), grid[0].len()];

        fn bfs(grid: &mut Vec<Vec<i32>>, pos: (i32, i32)) -> i32 {
            let [n, m] = [grid.len() as i32, grid[0].len() as i32];

            if pos.0 < 0 || pos.0 >= n || pos.1 < 0 || pos.1 >= m {
                return 0;
            }

            if grid[pos.0 as usize][pos.1 as usize] == 0 {
                return 0;
            }

            grid[pos.0 as usize][pos.1 as usize] = 0;

            let mut size = 1;

            let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            for dir in dirs.iter() {
                let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                size += bfs(grid, new_pos);
            }

            size
        }

        let mut grid = grid;
        let mut biggest_island_size = 0;

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    let island_size = bfs(&mut grid, (i as i32, j as i32));
                    biggest_island_size = i32::max(biggest_island_size, island_size);
                }
            }
        }

        biggest_island_size
    }
}

struct Solution {}
