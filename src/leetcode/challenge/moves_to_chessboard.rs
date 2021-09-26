impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();

        for i in 0..n {
            for j in 0..n {
                if board[0][0] ^ board[i][0] ^ board[0][j] ^ board[i][j] == 1 {
                    return -1;
                }
            }
        }

        let [mut row_sum, mut col_sum, mut row_misplaced, mut col_misplaced] = [0, 0, 0, 0];
        for i in 0..n {
            row_sum += board[0][i];
            col_sum += board[i][0];

            if board[i][0] == i as i32 % 2 {
                row_misplaced += 1;
            }
            if board[0][i] == i as i32 % 2 {
                col_misplaced += 1;
            }
        }

        let n = n as i32;
        if row_sum != n / 2 && row_sum != (n + 1) / 2 {
            return -1;
        }
        if col_sum != n / 2 && col_sum != (n + 1) / 2 {
            return -1;
        }

        if n % 2 == 1 {
            if col_misplaced % 2 == 1 {
                col_misplaced = n - col_misplaced;
            }
            if row_misplaced % 2 == 1 {
                row_misplaced = n - row_misplaced;
            }
        } else {
            col_misplaced = i32::min(n - col_misplaced, col_misplaced);
            row_misplaced = i32::min(n - row_misplaced, row_misplaced);
        }

        (col_misplaced + row_misplaced) / 2
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_moves_to_chessboard() {
        assert_eq!(Solution::moves_to_chessboard(vec_vec_i32![[0, 1, 1, 0], [0, 1, 1, 0], [1, 0, 0, 1], [1, 0, 0, 1]]), 2);
    }

    #[test]
    fn test_moves_to_chessboard2() {
        assert_eq!(Solution::moves_to_chessboard(vec_vec_i32![[0, 1], [1, 0]]), 0);
    }

    #[test]
    fn test_moves_to_chessboard3() {
        assert_eq!(Solution::moves_to_chessboard(vec_vec_i32![[1, 0], [1, 0]]), -1);
    }
}
