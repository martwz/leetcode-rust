use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if word.len() == 0 {
            return true;
        }

        let [n, m] = [board.len(), board[0].len()];

        fn dfs(board: &Vec<Vec<char>>, word: &String, idx: i32, pos: (i32, i32), seen: &mut Vec<Vec<i32>>, ans: &mut bool) {
            let [n, m] = [board.len(), board[0].len()];
            let (i, j) = pos;

            if idx >= word.len() as i32 {
                *ans = true;
                return;
            }

            if 0 > i || i >= n as i32 || 0 > j || j >= m as i32 {
                return;
            }
            if seen[i as usize][j as usize] == 1 {
                return;
            }
            if board[i as usize][j as usize] != word.chars().nth(idx as usize).unwrap() {
                return;
            }

            seen[i as usize][j as usize] = 1;
            let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            for dir in dirs.iter() {
                let pos = (i + dir.0, j + dir.1);
                dfs(board, word, idx + 1, (i + dir.0, j + dir.1), seen, ans);
            }
            seen[i as usize][j as usize] = 0;
        }

        let mut ans = false;
        for i in 0..n {
            for j in 0..m {
                let [i, j] = [i as i32, j as i32];
                let mut seen = vec![vec![0; m]; n];
                dfs(&board, &word, 0, (i, j), &mut seen, &mut ans);
            }
        }

        ans
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_vec_char, vec_vec_i32};

    #[test]
    fn test_exist() {
        assert_eq!(
            Solution::exist(vec_vec_char![['A', 'B', 'C', 'E'], ['S', 'F', 'C', 'S'], ['A', 'D', 'E', 'E']], "ABCCED".to_string()),
            true
        );
    }

    #[test]
    fn test_exist2() {
        assert_eq!(
            Solution::exist(vec_vec_char![['A', 'B', 'C', 'E'], ['S', 'F', 'C', 'S'], ['A', 'D', 'E', 'E']], "SEE".to_string()),
            true
        );
    }

    #[test]
    fn test_exist3() {
        assert_eq!(
            Solution::exist(vec_vec_char![['A', 'B', 'C', 'E'], ['S', 'F', 'C', 'S'], ['A', 'D', 'E', 'E']], "ABCB".to_string()),
            false
        );
    }

    #[test]
    fn test_exist4() {
        assert_eq!(Solution::exist(vec_vec_char![['a']], "a".to_string()), true);
    }
}
