use std::collections::{HashMap, VecDeque};

// 496. Next Greater Element I, Easy
// https://leetcode.com/problems/next-greater-element-i/
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut nums2_next_bigger = HashMap::new();

        for (i, curr_num) in nums2.iter().enumerate() {
            while queue.len() > 0 && *queue.back().unwrap() < curr_num {
                let num = queue.pop_back().unwrap();
                nums2_next_bigger.insert(num, *curr_num);
            }
            queue.push_back(curr_num);
        }

        queue.iter().for_each(|k| {
            nums2_next_bigger.insert(*k, -1);
        });

        let mut ans = vec![];

        'outer: for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                if nums1[i] == nums2[j] {
                    ans.push(*nums2_next_bigger.get(&nums2[j]).unwrap());
                    continue 'outer;
                }
            }
        }

        ans
    }

    pub fn next_greater_element_slow(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        'outer: for i in 0..nums1.len() {
            let mut found = -1;
            let mut found_i = -1;

            for j in i + 1..nums2.len() {
                if nums1[i] == nums2[j] {
                    found = nums2[j];
                    found_i = j as i32;
                    break;
                }
            }

            if found_i != -1 {
                let found_i = found_i as usize;
                for i in found_i + 1..nums2.len() {
                    if found < nums2[i] {
                        ans.push(nums2[i]);
                        continue 'outer;
                    }
                }
            }

            ans.push(-1);
        }

        ans
    }
}

struct Solution {}
