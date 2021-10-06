// 31. Next Permutation, Medium
// https://leetcode.com/problems/next-permutation/
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();

        if n <= 1 {
            return;
        }

        let mut swapped_at = 0;
        'outer: for l in (0..n).rev() {
            for r in (0..n).rev() {
                if r > l && nums[r] > nums[l] {
                    nums.swap(l, r);
                    swapped_at = l + 1;
                    break 'outer;
                }
            }
        }

        let mut i = swapped_at;
        while i < n - 1 {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                i = swapped_at;
                continue;
            }
            i += 1;
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_next_permutation() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn test_next_permutation2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_next_permutation3() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }

    #[test]
    fn test_next_permutation4() {
        let mut nums = vec![1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_next_permutation5() {
        let mut nums = vec![1, 3, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1, 3]);
    }

    #[test]
    fn test_next_permutation6() {
        let mut nums = vec![4, 2, 0, 2, 3, 2, 0];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![4, 2, 0, 3, 0, 2, 2]);
    }
}
