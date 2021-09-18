use std::cmp::Ordering;
use std::collections::HashMap;

// https://leetcode.com/problems/intersection-of-two-arrays-ii/
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums1.iter() {
            *map.entry(*num).or_insert(0) += 1
        }

        for num in nums2.iter() {
            if *map.get(&num).unwrap_or(&0) > 0 {
                *map.entry(*num).or_insert(0) -= 1;
                ans.push(*num);
            }
        }

        return ans;
    }

    pub fn intersect_using_pointers(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        nums1.sort();
        nums2.sort();

        let [mut i, mut j, n, m] = [0, 0, nums1.len(), nums2.len()];
        while i < n && j < m {
            match nums1[i].cmp(&nums2[j]) {
                Ordering::Less => i += 1,
                Ordering::Greater => j += 1,
                Ordering::Equal => {
                    ans.push(nums1[i]);
                    i += 1;
                    j += 1;
                }
            }
        }

        return ans;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        assert_eq!(Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    }

    #[test]
    fn test_intersect2() {
        assert_eq!(Solution::intersect(vec![1, 2, 2, 1], vec![]), vec![]);
    }

    #[test]
    fn test_intersect3() {
        assert_eq!(Solution::intersect(vec![], vec![]), vec![]);
    }

    #[test]
    fn intersect_using_pointers() {
        assert_eq!(Solution::intersect_using_pointers(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
    }

    #[test]
    fn intersect_using_pointers2() {
        assert_eq!(Solution::intersect_using_pointers(vec![1, 2, 2, 1], vec![]), vec![]);
    }

    #[test]
    fn intersect_using_pointers3() {
        assert_eq!(Solution::intersect_using_pointers(vec![], vec![]), vec![]);
    }
}
