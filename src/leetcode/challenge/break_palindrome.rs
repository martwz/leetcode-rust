// 1328. Break a Palindrome, Medium
// https://leetcode.com/problems/break-a-palindrome/
impl Solution {
    pub fn break_palindrome(mut palindrome: String) -> String {
        let n = palindrome.len();

        // can't break palindrom of length 1
        if palindrome.len() == 1 {
            return "".to_string();
        }

        // is palindrom?
        for i in 0..n / 2 {
            if palindrome.chars().nth(i).unwrap() != palindrome.chars().nth(n - i - 1).unwrap() {
                return "".to_string();
            }
        }

        // break palindrom
        let mut swapped = false;
        for i in 0..n {
            if palindrome.chars().nth(i).unwrap() != 'a' && !(n % 2 == 1 && i == n / 2) {
                palindrome.replace_range(i..i + 1, "a".to_string().as_str());
                swapped = true;
                break;
            }
        }

        if !swapped {
            palindrome.replace_range(n - 1..n, "b".to_string().as_str());
        }

        palindrome
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_palindrome() {
        assert_eq!(Solution::break_palindrome("abccba".to_string()), "aaccba".to_string());
    }

    #[test]
    fn test_break_palindrome2() {
        assert_eq!(Solution::break_palindrome("a".to_string()), "".to_string());
    }

    #[test]
    fn test_break_palindrome3() {
        assert_eq!(Solution::break_palindrome("aa".to_string()), "ab".to_string());
    }

    #[test]
    fn test_break_palindrome4() {
        assert_eq!(Solution::break_palindrome("aba".to_string()), "abb".to_string());
    }
}
