// 1275. Find Winner on a Tic Tac Toe Game, Easy
// https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut board = vec![vec![-1; 3]; 3];

        let mut player: i8 = 0;
        for m in moves.iter() {
            board[m[0] as usize][m[1] as usize] = player;

            // check for win
            for i in 0..3 {
                let mut row = vec![];
                let mut col = vec![];
                for j in 0..3 {
                    if board[i][j] == player {
                        row.push(board[i][j]);
                    }
                    if board[j][i] == player {
                        col.push(board[j][i]);
                    }
                }
                if row.len() == 3 || col.len() == 3 {
                    if player == 0 {
                        return "A".to_string();
                    } else {
                        return "B".to_string();
                    }
                }
                // diagonal
                if player == board[1][1] && player == board[2][2] && player == board[0][0] {
                    if player == 0 {
                        return "A".to_string();
                    } else {
                        return "B".to_string();
                    }
                }
                if player == board[1][1] && player == board[0][2] && player == board[2][0] {
                    if player == 0 {
                        return "A".to_string();
                    } else {
                        return "B".to_string();
                    }
                }
            }

            player ^= 1;
        }

        if moves.len() == 9 {
            return "Draw".to_string();
        } else {
            return "Pending".to_string();
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_tictactoe() {
        assert_eq!(Solution::tictactoe(vec_vec_i32![[0, 0], [2, 0], [1, 1], [2, 1], [2, 2]]), "A");
    }

    #[test]
    fn test_tictactoe2() {
        assert_eq!(Solution::tictactoe(vec_vec_i32![[0, 0], [1, 1], [0, 1], [0, 2], [1, 0], [2, 0]]), "B");
    }

    #[test]
    fn test_tictactoe3() {
        assert_eq!(
            Solution::tictactoe(vec_vec_i32![[0, 0], [1, 1], [2, 0], [1, 0], [1, 2], [2, 1], [0, 1], [0, 2], [2, 2]]),
            "Draw"
        );
    }

    #[test]
    fn test_tictactoe4() {
        assert_eq!(Solution::tictactoe(vec_vec_i32![[0, 0], [1, 1]]), "Pending");
    }
}
