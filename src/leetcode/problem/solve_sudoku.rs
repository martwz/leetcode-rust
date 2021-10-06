// 37. Sudoku Solver, Hard
// https://leetcode.com/problems/sudoku-solver/
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn get_valid_options(board: &mut Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> Vec<char> {
            let mut row = ['.'; 9];
            let mut col = ['.'; 9];
            let mut block = ['.'; 9];

            for i in 0..9 {
                row[i] = board[row_idx][i];
                col[i] = board[i][col_idx];
                block[i] = board[(row_idx / 3) * 3 + i / 3][(col_idx / 3) * 3 + i % 3];
            }

            let mut options = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
            for i in 0..9 {
                if row[i] != '.' {
                    options.retain(|&x| x != row[i]);
                }
                if col[i] != '.' {
                    options.retain(|&x| x != col[i]);
                }
                if block[i] != '.' {
                    options.retain(|&x| x != block[i]);
                }
            }

            options
        }

        fn backtrack(board_end: &mut Vec<Vec<char>>, board: &mut Vec<Vec<char>>, i: usize, j: usize) {
            if i == 9 {
                *board_end = board.clone();
                return;
            }

            if j == 9 {
                backtrack(board_end, board, i + 1, 0);
                return;
            }

            if board[i][j] != '.' {
                backtrack(board_end, board, i, j + 1);
                return;
            }

            let options = get_valid_options(board, i, j);
            for option in options {
                board[i][j] = option;
                backtrack(board_end, board, i, j + 1);
                board[i][j] = '.';
            }
        }

        let mut board_end = board.clone();
        backtrack(board, &mut board_end, 0, 0);
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_vec_char, vec_vec_i32};

    #[test]
    fn test_solve_sudoku() {
        let mut board = vec_vec_char![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            vec_vec_char![
                ['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                ['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                ['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                ['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                ['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                ['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                ['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                ['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                ['3', '4', '5', '2', '8', '6', '1', '7', '9']
            ]
        );
    }
}
