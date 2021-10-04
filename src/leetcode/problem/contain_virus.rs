use std::collections::VecDeque;

// 749. Contain Virus, Hard
// https://leetcode.com/problems/contain-virus/
impl Solution {
    pub fn contain_virus(mut is_infected: Vec<Vec<i32>>) -> i32 {
        fn find_regions(grid: &mut Vec<Vec<i32>>, start: (i32, i32)) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
            let mut regions = Vec::<(i32, i32)>::new();
            let mut neighbors = Vec::<(i32, i32)>::new();
            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

            let mut q = VecDeque::<(i32, i32)>::new();
            q.push_back(start);

            while !q.is_empty() {
                let pos = q.pop_back().unwrap();

                if grid[pos.0 as usize][pos.1 as usize] == 1 {
                    grid[pos.0 as usize][pos.1 as usize] = 2;
                    regions.push(pos);
                    for direction in directions.iter() {
                        let next = (pos.0 + direction.0, pos.1 + direction.1);
                        if next.0 < 0 || next.1 < 0 || next.0 >= grid.len() as i32 || next.1 >= grid[0].len() as i32 {
                            continue;
                        }
                        q.push_back(next);
                    }
                } else if grid[pos.0 as usize][pos.1 as usize] == 0 {
                    neighbors.push(pos);
                }
            }

            (regions, neighbors)
        }

        let mut walls = 0;
        let [n, m] = [is_infected.len(), is_infected[0].len()];
        let mut sim = true;

        while sim {
            sim = false;

            let mut state = is_infected.clone();
            let mut state_regions = Vec::<(Vec<(i32, i32)>, Vec<(i32, i32)>)>::new();
            for i in 0..n {
                for j in 0..m {
                    if state[i][j] == 1 {
                        let (regions, neighbors) = find_regions(&mut state, (i as i32, j as i32));
                        state_regions.push((regions, neighbors));
                    }
                }
            }

            // find biggest threat
            state_regions.sort_unstable_by(|a, b| a.1.len().cmp(&b.1.len()));

            // build walls
            if !state_regions.is_empty() {
                sim = true;
                let biggest_threat = state_regions.pop().unwrap();
                walls += biggest_threat.1.len() as i32;
                for pos in biggest_threat.0 {
                    is_infected[pos.0 as usize][pos.1 as usize] = 2;
                }
            }

            // spread the virus
            for threat in state_regions {
                for pos in threat.1 {
                    is_infected[pos.0 as usize][pos.1 as usize] = 1;
                }
            }
        }

        walls
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_is_infected() {
        assert_eq!(
            Solution::contain_virus(vec_vec_i32![
                [0, 1, 0, 0, 0, 0, 0, 1],
                [0, 1, 0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0, 0, 0]
            ]),
            10
        );
    }

    #[test]
    fn test_is_infected2() {
        assert_eq!(Solution::contain_virus(vec_vec_i32![[1, 1, 1], [1, 0, 1], [1, 1, 1]]), 4);
    }

    #[test]
    fn test_is_infected3() {
        assert_eq!(
            Solution::contain_virus(vec_vec_i32![[1, 1, 1, 0, 0, 0, 0, 0, 0], [1, 0, 1, 0, 1, 1, 1, 1, 1], [1, 1, 1, 0, 0, 0, 0, 0, 0]]),
            13
        );
    }

    #[test]
    fn test_is_infected4() {
        assert_eq!(
            Solution::contain_virus(vec_vec_i32![[0, 1, 0, 0, 0, 0, 0, 1], [0, 1, 0, 1, 0, 0, 0, 1], [0, 0, 0, 0, 0, 0, 0, 1]]),
            16
        );
    }
}
