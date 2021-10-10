// 201. Bitwise AND of Numbers Range, Medium
// https://leetcode.com/problems/bitwise-and-of-numbers-range/
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let [mut left, mut right] = [left, right];
        let mut shifts = 0;

        while left < right {
            left >>= 1;
            right >>= 1;
            shifts += 1;
        }

        left << shifts
    }

    pub fn range_bitwise_and_slow(left: i32, right: i32) -> i32 {
        let mut ans = left;

        if left < right {
            ((left + 1)..=right).for_each(|num| {
                println!("{:?}", num);
                ans &= num;
            });
        }

        ans
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_bitwise_and() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    }

    #[test]
    fn test_range_bitwise_and2() {
        assert_eq!(Solution::range_bitwise_and(2147483647, 2147483647), 2147483647);
    }

    #[test]
    fn test_range_bitwise_and3() {
        assert_eq!(Solution::range_bitwise_and(1, 2), 0);
    }

    #[test]
    fn test_range_bitwise_and_slow() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    }

    #[test]
    fn test_range_bitwise_and_slow2() {
        assert_eq!(Solution::range_bitwise_and(2147483647, 2147483647), 2147483647);
    }

    #[test]
    fn test_range_bitwise_and_slow3() {
        assert_eq!(Solution::range_bitwise_and(1, 2), 0);
    }
}
