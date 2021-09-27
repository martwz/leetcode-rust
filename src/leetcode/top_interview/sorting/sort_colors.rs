// 75. Sort Colors, Medium
// https://leetcode.com/problems/sort-colors/
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n <= 1 {
            return;
        }

        let mut swapped = true;

        while swapped {
            swapped = false;
            for i in 0..n - 1 {
                if nums[i] > nums[i + 1] {
                    nums.swap(i, i + 1);
                    swapped = true;
                    break;
                }
            }
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_i32, vec_vec_string};

    #[test]
    fn test_sort_colors() {
        let mut colors = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut colors);
        assert_eq!(colors, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_sort_colors2() {
        let mut colors = vec![2, 0, 1];
        Solution::sort_colors(&mut colors);
        assert_eq!(colors, vec![0, 1, 2]);
    }

    #[test]
    fn test_sort_colors3() {
        let mut colors = vec![0];
        Solution::sort_colors(&mut colors);
        assert_eq!(colors, vec![0]);
    }

    #[test]
    fn test_sort_colors4() {
        let mut colors = vec![1];
        Solution::sort_colors(&mut colors);
        assert_eq!(colors, vec![1]);
    }
}
