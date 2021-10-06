// 442. Find All Duplicates in an Array, Medium
// https://leetcode.com/problems/find-all-duplicates-in-an-array/
impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        let mut ans = vec![];
        let n = nums.len();
        if n == 0 {
            return ans;
        }

        let mut prev = nums[0];

        for i in 1..n {
            if nums[i] == prev {
                ans.push(nums[i]);
            }
            prev = nums[i];
        }

        ans
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates() {
        assert_eq!(Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), vec![2, 3]);
    }

    #[test]
    fn test_find_duplicates2() {
        assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1]);
    }

    #[test]
    fn test_find_duplicates3() {
        assert_eq!(Solution::find_duplicates(vec![1]), vec![]);
    }
}
