// 901. Online Stock Span, Medium
// https://leetcode.com/problems/online-stock-span/
impl StockSpanner {
    fn new() -> Self {
        StockSpanner { prices: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        self.prices.push(price);

        let mut i = 0;
        for d in (0..self.prices.len()).rev() {
            if self.prices[d] <= price {
                i += 1;
            }
        }

        i
    }
}

struct StockSpanner {
    prices: Vec<i32>,
}
