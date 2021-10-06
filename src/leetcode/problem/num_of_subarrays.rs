// @leetup=custom
// @leetup=info id=1524 lang=rust slug=number-of-sub-arrays-with-odd-sum

/*
* Given an array of integers `arr`, return *the number of subarrays with an
* odd sum*.
*
* Since the answer can be very large, return it modulo `109 + 7`.
*
*
* Example 1:
*
* Input: arr = [1,3,5]
* Output: 4
* Explanation: All subarrays are [[1],[1,3],[1,3,5],[3],[3,5],[5]]
* All sub-arrays sum are [1,4,9,3,8,5].
* Odd sums are [1,9,3,5] so the answer is 4.
*
* Example 2:
*
* Input: arr = [2,4,6]
* Output: 0
* Explanation: All subarrays are [[2],[2,4],[2,4,6],[4],[4,6],[6]]
* All sub-arrays sum are [2,6,12,4,10,6].
* All sub-arrays have even sum and the answer is 0.
*
* Example 3:
*
* Input: arr = [1,2,3,4,5,6,7]
* Output: 16
*
*
* Constraints:
*
* * `1 <= arr.length <= 105`
* * `1 <= arr[i] <= 100`
*
*/
// @leetup=custom
// @leetup=code

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let [mut even, mut odd] = [1, 0];
        let mut sum = 0;
        let mut ans = 0;
        for num in arr.iter() {
            sum += num;

            if sum % 2 == 0 {
                even += 1;
                ans += odd % MOD
            } else {
                odd += 1;
                ans += even % MOD
            }
        }

        ans
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_char};

    #[test]
    fn test_num_of_subarrays() {
        assert_eq!(Solution::num_of_subarrays(vec![1, 3, 5]), 4);
    }

    #[test]
    fn test_num_of_subarrays2() {
        assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0);
    }

    #[test]
    fn test_num_of_subarrays3() {
        assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
    }
}
