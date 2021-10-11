// 56. Merge Intervals, Medium
// https://leetcode.com/problems/merge-intervals/
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut ans: Vec<Vec<i32>> = vec![];

        for interval in intervals.iter_mut() {
            if ans.len() == 0 {
                ans.push(interval.clone());
            } else {
                if interval[0] >= ans.last().unwrap()[0] && interval[1] <= ans.last().unwrap()[1] {
                    continue;
                } else if interval[0] <= ans.last().unwrap()[1] {
                    ans.last_mut().unwrap()[1] = interval[1];
                } else {
                    ans.push(interval.clone());
                }
            }
        }

        ans
    }
}

struct Solution {}

mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_i32};

    #[test]
    fn test_merge() {
        assert_eq!(Solution::merge(vec_vec_i32![[1, 3], [2, 6], [8, 10], [15, 18]]), vec_vec_i32![[1, 6], [8, 10], [15, 18]]);
    }

    #[test]
    fn test_merge2() {
        assert_eq!(Solution::merge(vec_vec_i32![[1, 4], [4, 5]]), vec_vec_i32![[1, 5]]);
    }
}
