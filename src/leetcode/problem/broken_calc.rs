// 991. Broken Calculator, Medium
// https://leetcode.com/problems/broken-calculator/
impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        let mut ops = 0;
        let mut value = target;

        while value > start_value {
            if value % 2 == 1 {
                value += 1;
                ops += 1;
            }

            value /= 2;
            ops += 1;
        }

        ops += start_value - value;
        ops
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_broken_calc() {
        assert_eq!(Solution::broken_calc(2, 3), 2);
    }

    #[test]
    fn test_broken_calc2() {
        assert_eq!(Solution::broken_calc(5, 8), 2);
    }

    #[test]
    fn test_broken_calc3() {
        assert_eq!(Solution::broken_calc(3, 10), 3);
    }

    #[test]
    fn test_broken_calc4() {
        assert_eq!(Solution::broken_calc(1024, 1), 1023);
    }

    #[test]
    fn test_broken_calc5() {
        assert_eq!(Solution::broken_calc(1, 1000000000), 39);
    }
}
