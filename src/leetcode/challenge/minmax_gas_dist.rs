// 774. Minimize Max Distance to Gas Station, Medium
// https://leetcode.com/problems/minimize-max-distance-to-gas-station/
impl Solution {
    pub fn minmax_gas_dist(stations: Vec<i32>, k: i32) -> f64 {
        fn is_valid(stations: &Vec<i32>, k: i32, mid: f64) -> bool {
            let mut count = 0;

            (1..stations.len()).for_each(|i| {
                count += ((stations[i] - stations[i - 1]) as f64 / mid) as i32;
            });

            count <= k
        }

        let [mut l, mut r] = [0.0, 1e8];
        while r - l > 1e-6 {
            let mid = (l + r) / 2.0;
            if is_valid(&stations, k, mid) {
                r = mid;
            } else {
                l = mid;
            }
        }

        l
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minmax_gas_dist() {
        assert!(0.4 < Solution::minmax_gas_dist(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 9) && Solution::minmax_gas_dist(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 9) < 0.6,);
    }
}
