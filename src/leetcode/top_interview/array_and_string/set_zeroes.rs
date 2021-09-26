// 73. Set Matrix Zeroes, Medium
// https://leetcode.com/problems/set-matrix-zeroes/submissions/
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let [n, m] = [matrix.len(), matrix[0].len()];
        let mut zeros: Vec<(usize, usize)> = Vec::new();

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 0 {
                    zeros.push((i, j));
                }
            }
        }

        for (zi, zj) in zeros {
            for i in 0..n {
                matrix[i][zj] = 0;
            }
            for j in 0..m {
                matrix[zi][j] = 0;
            }
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_set_zeroes() {
        let mut matrix = vec_vec_i32![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec_vec_i32![[1, 0, 1], [0, 0, 0], [1, 0, 1]]);
    }

    #[test]
    fn test_set_zeroes2() {
        let mut matrix = vec_vec_i32![[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec_vec_i32![[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]);
    }
}
