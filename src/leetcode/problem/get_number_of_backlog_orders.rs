use std::collections::BinaryHeap;

// 1801. Number of Orders in the Backlog, Medium
// https://leetcode.com/problems/number-of-orders-in-the-backlog/
impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        #[derive(Eq, Debug, Clone)]
        struct Order {
            price: i32,
            amount: i32,
            order_type: i32,
        }

        impl Order {
            fn new(mut price: i32, amount: i32, order_type: i32) -> Self {
                if order_type == 1 {
                    price *= -1;
                }

                Self { price, amount, order_type }
            }
        }

        impl Ord for Order {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.price.cmp(&other.price)
            }
        }

        impl PartialOrd for Order {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for Order {
            fn eq(&self, other: &Self) -> bool {
                self.price == other.price
            }
        }

        let base: i32 = 10;
        let modulo: f64 = (base.pow(9) + 7) as f64;

        let mut buy_backlog: BinaryHeap<Order> = BinaryHeap::new();
        let mut sell_backlog: BinaryHeap<Order> = BinaryHeap::new();

        for order in orders.iter() {
            let mut o = Order::new(order[0], order[1], order[2]);

            while o.amount > 0 {
                if o.order_type == 0 {
                    if !sell_backlog.is_empty() && sell_backlog.peek().unwrap().price * -1 <= o.price {
                        let mut sell_order = sell_backlog.pop().unwrap();
                        let amount = i32::min(sell_order.amount, o.amount);
                        o.amount -= amount;
                        sell_order.amount -= amount;

                        if sell_order.amount > 0 {
                            sell_backlog.push(sell_order);
                        }
                    } else {
                        buy_backlog.push(o.clone());
                        break;
                    }
                } else if !buy_backlog.is_empty() && buy_backlog.peek().unwrap().price >= o.price * -1 {
                    let mut buy_order = buy_backlog.pop().unwrap();
                    let amount = i32::min(buy_order.amount, o.amount);
                    o.amount -= amount;
                    buy_order.amount -= amount;

                    if buy_order.amount > 0 {
                        buy_backlog.push(buy_order);
                    }
                } else {
                    sell_backlog.push(o.clone());
                    break;
                }
            }
        }

        ((buy_backlog.iter().map(|order| order.amount as f64).sum::<f64>() + sell_backlog.iter().map(|order| order.amount as f64).sum::<f64>()) % modulo) as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_get_number_of_backlog_orders() {
        assert_eq!(Solution::get_number_of_backlog_orders(vec_vec_i32![[10, 5, 0], [15, 2, 1], [25, 1, 1], [30, 4, 0]]), 6);
    }

    #[test]
    fn test_get_number_of_backlog_orders2() {
        assert_eq!(
            Solution::get_number_of_backlog_orders(vec_vec_i32![[7, 1000000000, 1], [15, 3, 0], [5, 999999995, 0], [5, 1, 1]]),
            999999984
        );
    }

    #[test]
    fn test_get_number_of_backlog_orders3() {
        assert_eq!(Solution::get_number_of_backlog_orders(vec_vec_i32![[19, 28, 0], [9, 4, 1], [25, 15, 1]]), 39);
    }

    #[test]
    fn test_get_number_of_backlog_orders4() {
        assert_eq!(
            Solution::get_number_of_backlog_orders(vec_vec_i32![
                [944925198, 885003661, 0],
                [852263791, 981056352, 0],
                [16300530, 415829909, 0],
                [940927944, 713835606, 0],
                [606389372, 407474168, 1],
                [139563740, 85382287, 1],
                [700244880, 901922025, 1],
                [972900669, 15506445, 0],
                [576578542, 65339074, 0],
                [45972021, 293765308, 0],
                [464403992, 97750995, 0],
                [29659852, 536508041, 0],
                [799523481, 299864737, 0],
                [711908211, 480514887, 1],
                [354125407, 677598767, 1],
                [279004011, 688916331, 0],
                [263524013, 64622178, 0],
                [375395974, 460070320, 0],
                [971786816, 379275200, 1],
                [577774472, 214070125, 1],
                [987757349, 711231195, 0]
            ]),
            83062672
        );
    }
}
