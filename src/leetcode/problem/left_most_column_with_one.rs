#[derive(Copy, Clone)]
struct BinaryMatrix;

#[allow(unused_variables)]
impl BinaryMatrix {
    fn get(self, row: i32, col: i32) -> i32 {
        1
    }

    fn dimensions(self) -> Vec<i32> {
        vec![]
    }
}

// 1428. Leftmost Column with at Least a One, Medium
// https://leetcode.com/problems/leftmost-column-with-at-least-a-one/
impl Solution {
    pub fn left_most_column_with_one(binary_matrix: &BinaryMatrix) -> i32 {
        let dim: Vec<_> = binary_matrix.dimensions();

        let mut row = dim[0] - 1;
        let mut col = dim[1] - 1;

        let mut candidate: Option<i32> = None;

        while row >= 0 && col >= 0 {
            if binary_matrix.get(row, col) == 1 {
                candidate = Some(col);
                col -= 1;
            } else {
                row -= 1;
            }
        }

        candidate.unwrap_or(-1)
    }
}

struct Solution {}
