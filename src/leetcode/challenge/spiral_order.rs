// 54. Spiral Matrix, Medium
// https://leetcode.com/problems/spiral-matrix/
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        // edge cases:
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return ans;
        } else if matrix[0].len() == 1 {
            matrix.iter().flatten().for_each(|f| {
                ans.push(*f);
            });
            return ans;
        }

        let [mut n, mut m] = [matrix.len() - 1, matrix[0].len() - 1];
        let cells = (n + 1) * (m + 1);
        let mut i = 0;

        #[derive(PartialEq)]
        enum Direction {
            RIGHT,
            DOWN,
            LEFT,
            UP,
        }

        let mut direction = Direction::RIGHT;
        let mut pos = (0, 0);
        ans.push(matrix[pos.0][pos.1]);

        while i < cells - 1 {
            if m > 0 && direction == Direction::RIGHT {
                for _j in 0..m {
                    pos = (pos.0, pos.1 + 1);
                    ans.push(matrix[pos.0][pos.1]);
                }
                if i != 0 {
                    m -= 1;
                }
                direction = Direction::DOWN;
            } else if n > 0 && direction == Direction::DOWN {
                for _j in 0..n {
                    pos = (pos.0 + 1, pos.1);
                    ans.push(matrix[pos.0][pos.1]);
                }
                n = n - 1;
                direction = Direction::LEFT;
            } else if m > 0 && direction == Direction::LEFT {
                for _j in 0..m {
                    pos = (pos.0, pos.1 - 1);
                    ans.push(matrix[pos.0][pos.1]);
                }
                m -= 1;
                direction = Direction::UP;
            } else if n > 0 && direction == Direction::UP {
                for _j in 0..n {
                    pos = (pos.0 - 1, pos.1);
                    ans.push(matrix[pos.0][pos.1]);
                }
                n -= 1;
                direction = Direction::RIGHT;
            }

            i += 1;
        }

        return ans;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_order() {
        assert_eq!(Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_spiral_order2() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[test]
    fn test_spiral_order3() {
        assert_eq!(Solution::spiral_order(vec![vec![1]]), vec![1]);
    }

    #[test]
    fn test_spiral_order4() {
        assert_eq!(Solution::spiral_order(vec![vec![3], vec![2]]), vec![3, 2]);
    }

    #[test]
    fn test_spiral_order5() {
        assert_eq!(Solution::spiral_order(vec![vec![3, 2], vec![0, 1]]), vec![3, 2, 1, 0]);
    }

    #[test]
    fn test_spiral_order6() {
        assert_eq!(Solution::spiral_order(vec![]), vec![]);
    }

    #[test]
    fn test_spiral_order7() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]]),
            vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
        );
    }
}
