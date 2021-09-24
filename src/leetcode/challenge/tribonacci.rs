// https://leetcode.com/problems/n-th-tribonacci-number/
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut cache = vec![0; n as usize + 1];
        for i in 0..n + 1 {
            if i == 0 {
                cache[0] = 0;
            } else if i == 1 {
                cache[1] = 1;
            } else if i == 2 {
                cache[2] = 1;
            } else {
                cache[i as usize] = cache[i as usize - 3] + cache[i as usize - 2] + cache[i as usize - 1];
            }
        }
        println!("{:?}", cache);
        cache[n as usize]
    }
}
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_tribonacci() {
        assert_eq!(Solution::tribonacci(4), 4);
    }

    #[test]
    fn test_tribonacci2() {
        assert_eq!(Solution::tribonacci(25), 1389537);
    }
}
