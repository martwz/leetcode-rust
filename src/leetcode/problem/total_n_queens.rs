use std::collections::{HashSet, VecDeque};

// 52. N-Queens II, Hard
// https://leetcode.com/problems/n-queens-ii/
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut board: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];

        // Check if queen can be placed
        fn is_valid_placement(board: &Vec<Vec<i32>>, n: i32, i: i32, j: i32) -> bool {
            let mut queue = VecDeque::new();

            queue.push_back((i, j, (1, 0)));
            queue.push_back((i, j, (1, 1)));
            queue.push_back((i, j, (0, 1)));
            queue.push_back((i, j, (-1, 1)));
            queue.push_back((i, j, (-1, 0)));
            queue.push_back((i, j, (-1, -1)));
            queue.push_back((i, j, (0, -1)));
            queue.push_back((i, j, (1, -1)));

            while !queue.is_empty() {
                let (x, y, direction) = queue.pop_front().unwrap();

                if 0 <= x && x < n && 0 <= y && y < n {
                    if board[x as usize][y as usize] == 1 {
                        return false;
                    }

                    queue.push_back((x + direction.0, y + direction.1, direction));
                }
            }

            return true;
        }

        let mut ans: Vec<Vec<Vec<i32>>> = vec![];
        fn backtracking(ans: &mut Vec<Vec<Vec<i32>>>, board: &mut Vec<Vec<i32>>, n: usize, placed: usize, i: usize, j: usize) {
            if placed == n {
                ans.push(board.clone());
                return;
            } else if i == n {
                return;
            } else if j == n {
                backtracking(ans, board, n, placed, i + 1, 0);
            } else {
                if is_valid_placement(board, n as i32, i as i32, j as i32) {
                    board[i][j] = 1;
                    backtracking(ans, board, n, placed + 1, i, j + 1);
                    board[i][j] = 0;
                }

                backtracking(ans, board, n, placed, i, j + 1);
            }
        }

        backtracking(&mut ans, &mut board, n as usize, 0, 0, 0);

        ans.len() as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates() {
        assert_eq!(Solution::total_n_queens(4), 2);
    }

    #[test]
    fn test_find_duplicates2() {
        assert_eq!(Solution::total_n_queens(1), 1);
    }
}
