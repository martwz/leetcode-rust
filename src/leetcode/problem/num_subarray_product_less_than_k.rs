// https://leetcode.com/problems/subarray-product-less-than-k/
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut l: i32 = 0;
        let mut ans: i32 = 0;
        let mut product: i32 = 1;

        for r in 0..nums.len() {
            product *= nums[r];

            if product >= k {
                while product >= k && l <= r as i32 {
                    product /= nums[l as usize];
                    l += 1;
                }
            }

            ans += r as i32 - l as i32 + 1;
        }

        ans as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_subarray_product_less_than_k() {
        assert_eq!(Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100), 8);
    }

    #[test]
    fn num_subarray_product_less_than_k2() {
        assert_eq!(Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0), 0);
    }
}
