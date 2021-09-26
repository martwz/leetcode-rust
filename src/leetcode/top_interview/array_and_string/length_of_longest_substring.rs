// 3. Longest Substring Without Repeating Characters, Medium
// https://leetcode.com/problems/longest-substring-without-repeating-characters/
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;

        let mut q = Vec::new();
        for ch in s.chars() {
            while q.contains(&ch) {
                q.remove(0);
            }
            q.push(ch);
            max = max.max(q.len() as i32);
        }

        max
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn test_length_of_longest_substring2() {
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn test_length_of_longest_substring3() {
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }

    #[test]
    fn test_length_of_longest_substring4() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }
}
