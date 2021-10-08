use std::collections::HashSet;

// 1041. Robot Bounded In Circle, Medium
// https://leetcode.com/problems/robot-bounded-in-circle/submissions/
impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        #[derive(PartialEq, Eq)]
        enum Direction {
            North,
            East,
            South,
            West,
        }

        let mut pos = (0, 0);
        let mut direction = Direction::North;

        for instruction in instructions.chars() {
            // move
            match instruction {
                'G' => match direction {
                    Direction::North => pos.1 += 1,
                    Direction::East => pos.0 += 1,
                    Direction::South => pos.1 += -1,
                    Direction::West => pos.0 += -1,
                },
                'L' => match direction {
                    Direction::North => direction = Direction::West,
                    Direction::East => direction = Direction::North,
                    Direction::South => direction = Direction::East,
                    Direction::West => direction = Direction::South,
                },
                'R' => match direction {
                    Direction::North => direction = Direction::East,
                    Direction::East => direction = Direction::South,
                    Direction::South => direction = Direction::West,
                    Direction::West => direction = Direction::North,
                },
                _ => {}
            }
        }

        pos == (0, 0) || direction != Direction::North
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn test_is_robot_bounded() {
        assert_eq!(Solution::is_robot_bounded("GGLLGG".to_string()), true);
    }

    #[test]
    fn test_is_robot_bounded2() {
        assert_eq!(Solution::is_robot_bounded("GG".to_string()), false);
    }

    #[test]
    fn test_is_robot_bounded3() {
        assert_eq!(Solution::is_robot_bounded("GL".to_string()), true);
    }

    #[test]
    fn test_is_robot_bounded4() {
        assert_eq!(Solution::is_robot_bounded("GLGLGGLGL".to_string()), false);
    }
}
