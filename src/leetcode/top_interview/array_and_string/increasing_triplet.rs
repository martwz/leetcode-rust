use std::collections::VecDeque;

// 334. Increasing Triplet Subsequence, Medium
// https://leetcode.com/problems/increasing-triplet-subsequence/
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let [mut small, mut big] = [std::i32::MAX, std::i32::MAX];

        for num in nums {
            if num <= small {
                small = num;
            } else if num <= big {
                big = num;
            } else {
                return true;
            }
        }

        false
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_i32, vec_vec_string};

    #[test]
    fn test_increasing_triplet() {
        assert_eq!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]), true);
    }

    #[test]
    fn test_increasing_triplet2() {
        assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false);
    }

    #[test]
    fn test_increasing_triplet3() {
        assert_eq!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
    }

    #[test]
    fn test_increasing_triplet4() {
        assert_eq!(Solution::increasing_triplet(vec![20, 100, 10, 12, 5, 13]), true);
    }
}
