// https://leetcode.com/problems/max-consecutive-ones/
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut consecs = 0;
        let mut curr_consecs = 0;
        let mut prev: i32 = 1;
        for i in nums.iter() {
            if *i == 1 && prev == 1 {
                curr_consecs += 1;
            } else {
                prev = *i;
                curr_consecs = if *i == 1 { 1 } else { 0 };
            }

            consecs = consecs.max(curr_consecs);
        }

        consecs
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_find_max_consecutive_ones() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
    }

    #[test]
    fn test_find_max_consecutive_ones2() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
    }
}
