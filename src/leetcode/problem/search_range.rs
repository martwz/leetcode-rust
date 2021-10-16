// 34. Find First and Last Position of Element in Sorted Array, Medium
// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![];

        match nums.binary_search(&target) {
            Ok(pos) => {
                let mut i = pos;
                let mut j = pos;
                while i > 0 && nums[i] == target {
                    i -= 1;
                }
                while j < nums.len() - 1 && nums[i] == target {
                    j += 1;
                }
                ans.append(&mut vec![i as i32 + 1, j as i32 - 1]);
            }
            Err(_) => ans.append(&mut vec![-1, -1]),
        }

        ans
    }
}

struct Solution {}
