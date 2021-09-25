// 452. Minimum Number of Arrows to Burst Balloons, Medium
// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a, b| a[1].cmp(&b[1]));
        
        let mut rms = 1;
        let mut prev = 0;

        for i in 1..points.len() {
            if points[prev][1] < points[i][0] {
                rms += 1;
                prev = i;
            }
        }

        rms
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_find_min_arrow_shots() {
        assert_eq!(Solution::find_min_arrow_shots(vec_vec_i32![[10, 16], [2, 8], [1, 6], [7, 12]]), 2);
    }

    #[test]
    fn test_find_min_arrow_shots2() {
        assert_eq!(Solution::find_min_arrow_shots(vec_vec_i32![[1, 2], [3, 4], [5, 6], [7, 8]]), 4);
    }

    #[test]
    fn test_find_min_arrow_shots3() {
        assert_eq!(Solution::find_min_arrow_shots(vec_vec_i32![[1, 2], [2, 3], [3, 4], [4, 5]]), 2);
    }
}
