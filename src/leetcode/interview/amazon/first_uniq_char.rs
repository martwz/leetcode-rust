use std::collections::HashMap;

// 387. First Unique Character in a String, Easy
// https://leetcode.com/problems/first-unique-character-in-a-string/
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = HashMap::<char, i32>::new();

        for char in s.chars() {
            map.entry(char).and_modify(|f| *f += 1).or_insert(1);
        }

        for (i, char) in s.chars().enumerate() {
            if *map.get(&char).unwrap() == 1 {
                return i as i32;
            }
        }

        -1
    }
}

struct Solution {}
