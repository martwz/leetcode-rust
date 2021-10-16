// 123. Best Time to Buy and Sell Stock III, Hard
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 {
            return 0;
        }

        let [mut t1_cost, mut t2_cost] = [i32::MAX, i32::MAX];
        let [mut t1_profit, mut t2_profit] = [0, 0];

        for &price in prices.iter() {
            t1_cost = i32::min(t1_cost, price);
            t1_profit = i32::max(t1_profit, price - t1_cost);
            t2_cost = i32::min(t2_cost, price - t1_profit);
            t2_profit = i32::max(t2_profit, price - t2_cost);
        }
        
        t2_profit
    }
}

struct Solution {}
