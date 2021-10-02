use std::collections::HashMap;

// 741. Cherry Pickup, Hard
// https://leetcode.com/problems/cherry-pickup/
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        let mut memo: HashMap<(usize, usize, usize, usize), i32> = HashMap::new();

        fn dp(grid: &Vec<Vec<i32>>, n: &usize, i1: usize, j1: usize, i2: usize, j2: usize, memo: &mut HashMap<(usize, usize, usize, usize), i32>) -> i32 {
            if i1 >= *n || j1 >= *n || i2 >= *n || j2 >= *n {
                return -1;
            } else if i1 == *n - 1 && j1 == *n - 1 && i2 == *n - 1 && j2 == *n - 1 {
                return grid[i1][j1];
            } else if grid[i1][j1] == -1 || grid[i2][j2] == -1 {
                return -1;
            }

            let key = &(i1, j1, i2, j2);
            if memo.contains_key(key) {
                return *memo.get(key).unwrap();
            }

            let dd = dp(&grid, &n, i1 + 1, j1, i2 + 1, j2, memo);
            let dr = dp(&grid, &n, i1 + 1, j1, i2, j2 + 1, memo);
            let rd = dp(&grid, &n, i1, j1 + 1, i2 + 1, j2, memo);
            let rr = dp(&grid, &n, i1, j1 + 1, i2, j2 + 1, memo);

            let comb = i32::max(dd, i32::max(dr, i32::max(rd, rr)));

            let out;
            if comb == -1 {
                out = -1;
            } else {
                if i1 == i2 && j1 == j2 {
                    out = comb + grid[i1][j1];
                } else {
                    out = comb + grid[i1][j1] + grid[i2][j2];
                }
            }

            memo.insert(*key, out);
            out
        }

        i32::max(dp(&grid, &n, 0, 0, 0, 0, &mut memo), 0)
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_cherry_pickup() {
        assert_eq!(Solution::cherry_pickup(vec_vec_i32![[0, 1, -1], [1, 0, -1], [1, 1, 1]]), 5);
    }

    #[test]
    fn test_cherry_pickup2() {
        assert_eq!(Solution::cherry_pickup(vec_vec_i32![[1, 1, -1], [1, -1, 1], [-1, 1, 1]]), 0);
    }
}
