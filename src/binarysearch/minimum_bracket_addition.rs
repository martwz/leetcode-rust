use std::collections::{HashMap, VecDeque};

/*
  Minimum Bracket Addition
  https://binarysearch.com/problems/Minimum-Bracket-Addition
*/

impl Solution {
    pub fn minimum_bracket_addition(s: String) -> i32 {
        let mut queue: VecDeque<char> = VecDeque::new();

        let mut inserts = 0;
        for c in s.chars() {
            if c == '(' {
                queue.push_back(c);
            } else {
                if queue.back() == Some(&'(') {
                    queue.pop_back();
                } else {
                    inserts += 1;
                }
            }
        }

        return inserts + queue.len() as i32;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_bracket_addition() {
        assert_eq!(Solution::minimum_bracket_addition(")))((".to_string()), 5);
    }
}
