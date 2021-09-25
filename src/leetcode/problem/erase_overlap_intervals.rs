// 435. Non-overlapping Intervals, Medium
// https://leetcode.com/problems/non-overlapping-intervals/
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut rms = 0;
        let mut prev = 0;

        for i in 1..intervals.len() {
            if intervals[prev][1] > intervals[i][0] {
                rms += 1;
            } else {
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
    fn test_erase_overlap_intervals() {
        assert_eq!(Solution::erase_overlap_intervals(vec_vec_i32![[1, 2], [2, 3], [3, 4], [1, 3]]), 1);
    }

    #[test]
    fn test_erase_overlap_intervals2() {
        assert_eq!(Solution::erase_overlap_intervals(vec_vec_i32![[1, 2], [1, 2], [1, 2]]), 2);
    }

    #[test]
    fn test_erase_overlap_intervals3() {
        assert_eq!(Solution::erase_overlap_intervals(vec_vec_i32![[1, 2], [2, 3]]), 0);
    }
}
