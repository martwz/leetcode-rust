use std::collections::HashMap;

// 1481. Least Number of Unique Integers after K Removals, Medium
// https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut dict = HashMap::<i32, i32>::new();

        for num in arr.iter() {
            dict.entry(*num).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut list = dict.keys().map(|k| (*k, dict[k])).collect::<Vec<(i32, i32)>>();
        list.sort_unstable_by(|a, b| a.1.cmp(&b.1));

        let mut k = k;
        let mut i = 0;
        let mut num_uniques = list.len();

        while k > 0 {
            let can_rm = i32::min(k, list[i].1);

            if list[i].1 - can_rm == 0 {
                num_uniques -= 1;
                i += 1;
                k -= can_rm;
            } else {
                k -= can_rm;
            }
        }

        // let list =

        num_uniques as i32
    }
}

struct Solution {}
