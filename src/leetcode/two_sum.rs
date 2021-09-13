use std::collections::HashMap;

/*
  1. Two Sum
  https://leetcode.com/problems/two-sum/
*/

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let rest = target - num;
            if let Some(j) = map.get(&rest) {
                return vec![*j, i as i32];
            }
            map.insert(*num, i as i32);
        }

        return vec![];
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
