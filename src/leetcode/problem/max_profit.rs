// 309. Best Time to Buy and Sell Stock with Cooldown, Medium
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 {
            return 0;
        }

        let [mut sold, mut held, mut reset] = [i32::MIN, i32::MIN, 0];

        for price in prices.iter() {
            let pre_sold = sold;
            sold = held + price;
            held = i32::max(held, reset - price);
            reset = i32::max(reset, pre_sold)
        }

        i32::max(sold, reset)
    }
}

struct Solution {}
