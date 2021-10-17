use std::collections::{BinaryHeap, HashMap, VecDeque};

// 2042. Check if Numbers Are Ascending in a Sentence, Easy
// https://leetcode.com/problems/check-if-numbers-are-ascending-in-a-sentence/
impl Solution1 {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut s = s;
        s.push('#');
        let mut prev: u32 = 0;
        let mut curr_digit: u32 = 0;

        for ch in s.chars() {
            if ch.is_numeric() {
                curr_digit = curr_digit * 10 + ch.to_digit(10).unwrap();
            } else {
                if curr_digit != 0 {
                    if prev >= curr_digit {
                        return false;
                    }
                    prev = curr_digit;
                    curr_digit = 0;
                }
            }
        }

        true
    }
}

struct Bank {
    accounts: HashMap<i32, i64>,
}

// 2043. Simple Bank System, Medium
// https://leetcode.com/problems/simple-bank-system/
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        let mut hm = HashMap::new();
        for i in 0..balance.len() {
            hm.insert(i as i32 + 1, balance[i]);
        }

        Bank { accounts: hm }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if !self.accounts.contains_key(&account1) || !self.accounts.contains_key(&account2) {
            return false;
        } else {
            let avail = *self.accounts.get(&account1).unwrap_or(&0);
            if avail >= money {
                self.accounts.entry(account1).and_modify(|b| *b -= money);
                self.accounts.entry(account2).and_modify(|b| *b += money);
                return true;
            } else {
                return false;
            }
        }
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if !self.accounts.contains_key(&account) {
            return false;
        } else {
            self.accounts.entry(account).and_modify(|b| *b += money);
            return true;
        }
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if !self.accounts.contains_key(&account) {
            return false;
        } else {
            let avail = *self.accounts.get(&account).unwrap_or(&0);
            if avail >= money {
                self.accounts.entry(account).and_modify(|b| *b -= money);
                return true;
            } else {
                return false;
            }
        }
    }
}

// 2044. Count Number of Maximum Bitwise-OR Subsets, Medium
// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/
impl Solution3 {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut max_ors = 0;
        for num in nums.iter() {
            max_ors |= *num;
        }

        fn subset(nums: &Vec<i32>, count: &mut i32, i: i32, a: i32, b: i32) {
            if i < 0 {
                return;
            }
            if a == b | nums[i as usize] {
                *count += 1
            }
            subset(&nums, count, i - 1, a, b);
            subset(&nums, count, i - 1, a, b | nums[i as usize]);
        }

        let mut count = 0;
        subset(&nums, &mut count, nums.len() as i32 - 1, max_ors, 0);

        count
    }
}

// 2045. Second Minimum Time to Reach Destination, Hard
// https://leetcode.com/problems/second-minimum-time-to-reach-destination/
impl Solution4 {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let mut seen = vec![false; n + 1];
        seen[0] = true;

        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        queue.push_back((1, 0));
        seen[1] = true;

        let mut dists: Vec<Vec<i32>> = vec![vec![]; n + 1];

        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..=n {
            adj.insert(i as i32, vec![]);
        }
        for edge in edges.iter() {
            let (u, v) = (edge[0], edge[1]);
            adj.entry(u).and_modify(|e| e.push(v));
            adj.entry(v).and_modify(|e| e.push(u));
        }

        while !queue.is_empty() {
            let (vertex, elapsed) = queue.pop_front().unwrap();

            seen[vertex as usize] = true;

            for &e in adj.get(&vertex).unwrap() {
                let cand: i32;
                if elapsed as i32 / change % 2 == 0 {
                    cand = elapsed + time;
                } else {
                    cand = (f32::ceil(elapsed as f32 / (2.0 * change as f32)) * (2.0 * change as f32) + time as f32) as i32;
                }

                if dists[e as usize].is_empty() || (dists[e as usize].len() == 1 && dists[e as usize] != vec![cand]) {
                    dists[e as usize].push(cand);
                    queue.push_back((e, cand));
                }
            }
        }

        *dists[n].iter().max().unwrap()
    }
}

struct Solution1 {}
struct Solution3 {}
struct Solution4 {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_i32};

    #[test]
    fn test_second_minimum() {
        assert_eq!(Solution4::second_minimum(5, vec_vec_i32![[1, 2], [1, 3], [1, 4], [3, 4], [4, 5]], 3, 5), 13);
    }

    #[test]
    fn test_minimum_moves2() {
        assert_eq!(Solution4::second_minimum(2, vec_vec_i32![[1, 2]], 3, 2), 11);
    }
}
