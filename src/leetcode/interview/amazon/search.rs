// 33. Search in Rotated Sorted Array, Medium
// https://leetcode.com/problems/search-in-rotated-sorted-array/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums.iter().enumerate().map(|(i, n)| (*n, i)).collect::<Vec<(i32, usize)>>();
        nums.sort_unstable();

        let i = nums.binary_search_by(|p| p.0.cmp(&target));
        match i {
            Ok(i) => nums[i].1 as i32,
            Err(_) => -1,
        }
    }
}

struct Solution {}
