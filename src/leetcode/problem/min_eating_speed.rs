// 875. Koko Eating Bananas, Medium
// https://leetcode.com/problems/koko-eating-bananas/
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        if piles.len() == 0 {
            return 0;
        }

        let [mut lo, mut hi] = [0, i32::max_value()];

        fn can_eat(piles: &Vec<i32>, h: i32, k: i32) -> bool {
            let mut hours = 0;
            piles.iter().for_each(|p| {
                hours += f64::ceil(*p as f64 / k as f64) as i32;
            });
            return hours <= h;
        }

        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            if can_eat(&piles, h, mid) {
                hi = mid;
            } else {
                lo = mid;
            }
        }

        return hi;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_eating_speed() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }

    #[test]
    fn test_min_eating_speed2() {
        assert_eq!(Solution::min_eating_speed(vec![3], 3), 1);
    }

    #[test]
    fn test_min_eating_speed3() {
        assert_eq!(Solution::min_eating_speed(vec![], 3), 0);
    }

    #[test]
    fn test_min_eating_speed4() {
        assert_eq!(
            Solution::min_eating_speed(
                vec![
                    332484035, 524908576, 855865114, 632922376, 222257295, 690155293, 112677673, 679580077, 337406589, 290818316, 877337160, 901728858, 679284947, 688210097,
                    692137887, 718203285, 629455728, 941802184
                ],
                823855818
            ),
            14
        );
    }
}
