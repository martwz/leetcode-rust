// 1134. Armstrong Number, Easy
// https://leetcode.com/problems/armstrong-number/
impl Solution {
    pub fn is_armstrong(n: i32) -> bool {
        let mut digits = vec![];
        let mut tmp = n;
        while tmp > 0 {
            digits.push(tmp % 10);
            tmp = tmp / 10;
        }

        let mut sum = 0;
        for digit in digits.iter() {
            sum += digit.pow(digits.len() as u32);
        }

        return sum == n;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_armstrong() {
        assert_eq!(Solution::is_armstrong(153), true);
    }

    #[test]
    fn test_is_armstrong2() {
        assert_eq!(Solution::is_armstrong(123), false);
    }
}
