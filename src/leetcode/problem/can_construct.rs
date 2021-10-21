use std::collections::HashMap;

// 1400. Construct K Palindrome Strings, Medium
// https://leetcode.com/problems/construct-k-palindrome-strings/
impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut counter: HashMap<char, i32> = HashMap::new();

        let n = s.len() as i32;
        for ch in s.chars() {
            counter.entry(ch).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut odds = 0;

        for (_k, v) in counter {
            if v % 2 != 0 {
                odds += 1;
            }
        }

        return n >= k && odds <= k;
    }
}

struct Solution {}
