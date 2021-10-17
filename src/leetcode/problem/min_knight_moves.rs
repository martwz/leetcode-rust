use std::collections::{HashSet, VecDeque};

// 1197. Minimum Knight Moves, Medium
// https://leetcode.com/problems/minimum-knight-moves/
impl Solution {
    pub fn min_knight_moves(x: i32, y: i32) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(((0, 0), 0));

        let mut seen = HashSet::<(i32, i32)>::new();

        let knight_moves = vec![(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)];

        while !queue.is_empty() {
            let (pos, moves) = queue.pop_front().unwrap();

            if x == pos.0 && y == pos.1 {
                return moves;
            }

            for knight_move in &knight_moves {
                let new_pos = (pos.0 + knight_move.0, pos.1 + knight_move.1);
                if !seen.contains(&new_pos) {
                    seen.insert(new_pos);
                    queue.push_back((new_pos, moves + 1));
                }
            }
        }

        -1
    }
}

struct Solution {}
