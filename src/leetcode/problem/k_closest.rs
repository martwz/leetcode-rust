use std::cmp::Ordering;

// 973. K Closest Points to Origin, Medium
// https://leetcode.com/problems/k-closest-points-to-origin/
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut enhanced_points = points
            .iter()
            .map(|n| {
                let dist = f64::sqrt(f64::powi(n[0] as f64, 2) + f64::powi(n[1] as f64, 2));
                (n[0], n[1], dist)
            })
            .collect::<Vec<(i32, i32, f64)>>();
        enhanced_points.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal));

        enhanced_points.iter().take(k as usize).map(|n| vec![n.0, n.1]).collect()
    }
}

struct Solution {}
