// 698. Partition to K Equal Sum Subsets, Medium
// https://leetcode.com/problems/partition-to-k-equal-sum-subsets/
impl Solution {
  pub fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
      let sum = nums.iter().sum::<i32>();
      if sum % k != 0 {
          return false;
      }
      let target = sum / k;
      
      nums.sort();
      if nums[nums.len() - 1] > target {
          return false;
      }

      fn backtrack(nums: &Vec<i32>, target: i32, pos: usize, next: i32, used: &mut Vec<bool>) -> bool {
          if used.iter().all(|x| *x) {
              return true;
          }

          for i in pos..nums.len() {
              if used[i] || next + nums[i] > target {
                  continue;
              }

              let next = (next + nums[i]) % target;
              used[i] = true;
              if backtrack(nums, target, if next == 0 { 0 } else { i + 1 }, next, used) {
                  return true;
              }
              used[i] = false;
          }

          false
      }

      backtrack(&nums, target, 0, 0, &mut vec![false; nums.len()])
  }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_partition_k_subsets() {
        assert_eq!(Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4), true);
    }

    #[test]
    fn test_can_partition_k_subsets2() {
        assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3), false);
    }

    #[test]
    fn test_can_partition_k_subsets3() {
        assert_eq!(Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 0], 4), false);
    }
}
