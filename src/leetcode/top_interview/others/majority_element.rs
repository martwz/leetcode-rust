use std::collections::VecDeque;

// 169. Majority Element, Easy
// https://leetcode.com/problems/majority-element/solution/
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut cand: Option<i32> = None;

        for num in nums {
            if cand.is_none() || count == 0 {
                cand = Some(num);
                count = 1;
            } else if cand.unwrap() == num {
                count += 1;
            } else if count > 0 {
                count -= 1;
            }
        }

        return cand.unwrap();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn test_majority_element2() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn test_majority_element3() {
        assert_eq!(Solution::majority_element(vec![3, 3, 4]), 3);
    }
}
