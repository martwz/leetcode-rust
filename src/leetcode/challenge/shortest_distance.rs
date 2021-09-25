use std::collections::HashMap;
use std::collections::VecDeque;

// 317. Shortest Distance from All Buildings, Hard
// https://leetcode.com/problems/shortest-distance-from-all-buildings/
impl Solution {
    pub fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 {
        fn manhattan_distance(x0: i32, x1: i32, y0: i32, y1: i32) -> i32 {
            i32::abs(x0 - x1) + i32::abs(y0 - y1)
        }

        fn dfs(grid: &Vec<Vec<i32>>, x: i32, y: i32) -> Vec<(i32, i32)> {
            let mut dists: Vec<(i32, i32)> = vec![];

            let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
            let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
            let mut seen: Vec<(i32, i32)> = Vec::new();

            queue.push_back((x, y));

            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();

                if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
                    continue;
                } else if seen.contains(&(x, y)) {
                    continue;
                } else if grid[x as usize][y as usize] == 2 {
                    continue;
                } else if grid[x as usize][y as usize] == 1 {
                    dists.push((x, y));
                    seen.push((x, y));
                    continue;
                }

                seen.push((x, y));

                for (dx, dy) in directions.iter() {
                    let nx = x + dx;
                    let ny = y + dy;

                    queue.push_back((nx, ny));
                }
            }

            dists
        }

        let rows = grid.len();
        if rows == 0 {
            return -1;
        }
        let cols = grid[0].len();

        let mut dist_map: HashMap<i32, i32> = HashMap::new();
        let mut friends = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    friends += 1;
                }
                if grid[r][c] == 0 {
                    let dists = dfs(&grid, r as i32, c as i32);
                    let mut curr_dist = 0;
                    for dist in dists.iter() {
                        curr_dist += manhattan_distance(r as i32, dist.0, c as i32, dist.1);
                    }
                    println!("{:?}", curr_dist);
                    dist_map.insert(dists.len() as i32, curr_dist.min(*dist_map.get(&(dists.len() as i32)).unwrap_or(&std::i32::MAX)));
                }
            }
        }

        println!("{:?}", dist_map);

        *dist_map.get(&friends).unwrap_or(&-1)
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use crate::vec_vec_i32;

    use super::*;

    #[test]
    fn test_shortest_distance() {
        assert_eq!(Solution::shortest_distance(vec_vec_i32![[1, 0, 2, 0, 1], [0, 0, 0, 0, 0], [0, 0, 1, 0, 0]]), 7);
    }

    #[test]
    fn test_shortest_distance2() {
        assert_eq!(Solution::shortest_distance(vec_vec_i32![[1, 0]]), 1);
    }

    #[test]
    fn test_shortest_distance3() {
        assert_eq!(Solution::shortest_distance(vec_vec_i32![[1]]), -1);
    }

    #[test]
    fn test_shortest_distance4() {
        assert_eq!(Solution::shortest_distance(vec_vec_i32![[1, 2, 0]]), -1);
    }

    #[test]
    fn test_shortest_distance5() {
        assert_eq!(Solution::shortest_distance(vec_vec_i32![[0, 2, 1], [1, 0, 2], [0, 1, 0]]), -1);
    }

    #[test]
    fn test_shortest_distance6() {
        assert_eq!(Solution::shortest_distance(vec_vec_i32![[1, 1], [0, 1]]), -1);
    }
}
