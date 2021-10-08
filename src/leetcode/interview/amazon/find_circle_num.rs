// 547. Number of Provinces, Medium
// https://leetcode.com/problems/number-of-provinces/
impl Solution {
    pub fn find_circle_num(mut is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();

        let mut provinces = 0;
        fn bfs(i: usize, is_connected: &mut Vec<Vec<i32>>) {
            let edges = is_connected[i].clone();
            is_connected[i] = vec![];

            for (i, is_edge) in edges.iter().enumerate() {
                if *is_edge == 1 {
                    bfs(i as usize, is_connected);
                }
            }
        }

        for i in 0..n {
            if is_connected[i].len() > 0 {
                provinces += 1;
            }
            bfs(i, &mut is_connected);
        }

        provinces
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_find_circle_num() {
        assert_eq!(Solution::find_circle_num(vec_vec_i32![[1, 1, 0], [1, 1, 0], [0, 0, 1]]), 2);
    }

    #[test]
    fn test_find_circle_num2() {
        assert_eq!(Solution::find_circle_num(vec_vec_i32![[1, 0, 0], [0, 1, 0], [0, 0, 1]]), 3);
    }
}
