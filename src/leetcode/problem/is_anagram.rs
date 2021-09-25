// 242. Valid Anagram, Easy
// https://leetcode.com/problems/valid-anagram/
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();
        s_chars.sort();
        t_chars.sort();

        for i in 0..s_chars.len() {
            if (s_chars[i] as u8) != (t_chars[i] as u8) {
                return false;
            }
        }

        return true;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(Solution::is_anagram("anagram".to_string(), "nagaram".to_string()), true);
    }

    #[test]
    fn test_is_anagram2() {
        assert_eq!(Solution::is_anagram("rat".to_string(), "car".to_string()), false);
    }
}
