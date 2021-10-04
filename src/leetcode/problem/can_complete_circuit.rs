// 134. Gas Station, Medium
// https://leetcode.com/problems/gas-station/
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();

        for i in 0..n {
            let mut pos = i;
            let mut dist = 0;
            let mut tank = 0;

            while dist <= n {
                tank += gas[pos] - cost[pos];
                if tank < 0 {
                    break;
                } else {
                    pos += 1;
                    pos %= n;
                    dist += 1;
                }
            }

            if dist >= n {
                return i as i32;
            }
        }

        -1
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
    }

    #[test]
    fn test_can_complete_circuit2() {
        assert_eq!(Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
    }
}
