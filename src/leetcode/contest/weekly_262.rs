use std::collections::BinaryHeap;
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

// 2032. Two Out of Three, Easy
// https://leetcode.com/problems/two-out-of-three/
impl Solution1 {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut count_map = HashMap::<i32, i32>::new();

        let set1 = nums1.iter().collect::<HashSet<&i32>>();
        let set2 = nums2.iter().collect::<HashSet<&i32>>();
        let set3 = nums3.iter().collect::<HashSet<&i32>>();

        for num1 in set1.iter() {
            count_map.entry(**num1).and_modify(|n| *n += 1).or_insert(1);
        }
        for num2 in set2.iter() {
            count_map.entry(**num2).and_modify(|n| *n += 1).or_insert(1);
        }
        for num3 in set3.iter() {
            count_map.entry(**num3).and_modify(|n| *n += 1).or_insert(1);
        }

        let mut ans = vec![];
        for (k, v) in count_map.iter() {
            if *v >= 2 {
                ans.push(*k);
            }
        }

        ans
    }
}

// 2033. Minimum Operations to Make a Uni-Value Grid, Medium
// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/
impl Solution2 {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums = grid.iter().flatten().map(|n| *n).collect::<Vec<i32>>();

        let min = nums.iter().min().unwrap();
        for i in 0..nums.len() {
            if (nums[i] - min) % x != 0 {
                return -1;
            }
        }

        nums.sort_unstable();
        let median = nums[nums.len() / 2];

        let mut ans = 0;
        for i in 0..nums.len() {
            ans += i32::abs(nums[i] - median) / x;
        }
        ans
    }
}

struct StockPrice {
    prices: Vec<i32>,
    last: (i32, i32), // (time, price)
    prices_map: HashMap<i32, i32>,
}

// 2034. Stock Price Fluctuation, Medium
// https://leetcode.com/problems/stock-price-fluctuation/
impl StockPrice {
    fn new() -> Self {
        StockPrice {
            prices: Vec::<i32>::new(),
            last: (0, 0),
            prices_map: HashMap::new(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        if timestamp >= self.last.0 {
            self.last.0 = timestamp;
            self.last.1 = price;
        }

        let old_price = self.prices_map.get(&timestamp);
        if let Some(old_price) = old_price {
            let pos = self.prices.binary_search(old_price).unwrap_or_else(|e| e);
            self.prices.remove(pos);
        }

        let pos = self.prices.binary_search(&price).unwrap_or_else(|e| e);
        self.prices.insert(pos, price);

        self.prices_map.entry(timestamp).and_modify(|n| *n = price).or_insert(price);
    }

    fn current(&self) -> i32 {
        self.last.1
    }

    fn maximum(&mut self) -> i32 {
        *self.prices.last().unwrap_or(&0)
    }

    fn minimum(&mut self) -> i32 {
        *self.prices.first().unwrap_or(&0)
    }
}

struct Solution1 {}
struct Solution2 {}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_i32};

    #[test]
    fn test_two_out_of_three() {
        assert_eq!(Solution1::two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]).len(), 2);
    }

    #[test]
    fn test_min_operations() {
        assert_eq!(Solution2::min_operations(vec_vec_i32![[2, 4], [6, 8]], 2), 4);
    }

    #[test]
    fn test_min_operations2() {
        assert_eq!(Solution2::min_operations(vec_vec_i32![[1, 5], [2, 3]], 1), 5);
    }

    #[test]
    fn test_min_operations3() {
        assert_eq!(Solution2::min_operations(vec_vec_i32![[1, 2], [3, 4]], 2), -1);
    }

    #[test]
    fn test_min_operations4() {
        assert_eq!(Solution2::min_operations(vec_vec_i32![[146]], 86), 0);
    }

    #[test]
    fn test_min_operations5() {
        assert_eq!(Solution2::min_operations(vec_vec_i32![[931, 128], [639, 712]], 73), 12);
    }

    #[test]
    fn test_min_operations6() {
        assert_eq!(
            Solution2::min_operations(vec_vec_i32![[980, 476, 644, 56], [644, 140, 812, 308], [812, 812, 896, 560], [728, 476, 56, 812]], 84),
            45
        );
    }

    #[test]
    fn test_min_operations7() {
        assert_eq!(
            Solution2::min_operations(
                vec_vec_i32![
                    [454, 328, 160, 286, 664],
                    [496, 538, 748, 244, 286],
                    [34, 244, 454, 706, 790],
                    [496, 538, 832, 958, 328],
                    [370, 874, 370, 874, 286]
                ],
                42
            ),
            122
        );
    }

    #[test]
    fn test_min_operations8() {
        assert_eq!(
            Solution2::min_operations(
                vec_vec_i32![
                    [596, 904, 960, 232, 120, 932, 176],
                    [372, 792, 288, 848, 960, 960, 764],
                    [652, 92, 904, 120, 680, 904, 120],
                    [372, 960, 92, 680, 876, 624, 904],
                    [176, 652, 64, 344, 316, 764, 316],
                    [820, 624, 848, 596, 960, 960, 372],
                    [708, 120, 456, 92, 484, 932, 540]
                ],
                28
            ),
            473
        );
    }

    #[test]
    fn test_stock_price() {
        let mut obj = StockPrice::new();
        obj.update(1, 1);
        let ret_2: i32 = obj.current();
        let ret_3: i32 = obj.maximum();
        let ret_4: i32 = obj.minimum();
        assert_eq!(ret_2, 1);
        assert_eq!(ret_3, 1);
        assert_eq!(ret_4, 1);
    }
}
