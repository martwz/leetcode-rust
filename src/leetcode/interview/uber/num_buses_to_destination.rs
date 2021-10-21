use std::collections::{HashMap, HashSet, VecDeque};

// 815. Bus Routes, Hard
// https://leetcode.com/problems/bus-routes/
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        let mut hm: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

        for (bus, route) in routes.iter().enumerate() {
            let bus = bus as i32;

            let mut route = route.clone();
            route.push(route[0]);

            for j in 0..route.len() - 1 {
                let j = j as i32;
                hm.entry(route[j as usize])
                    .and_modify(|f| f.push((route[j as usize + 1], bus)))
                    .or_insert(vec![(route[j as usize + 1], bus)]);
            }
        }

        let mut queue: VecDeque<(i32, HashSet<i32>)> = VecDeque::new();
        queue.push_back((source, HashSet::<i32>::new()));

        let mut seen: HashSet<i32> = HashSet::new();

        let mut min_busses = usize::MAX;

        while !queue.is_empty() {
            let (stop, busses) = queue.pop_front().unwrap();

            if stop == target {
                min_busses = min_busses.min(busses.len());
            }

            seen.insert(stop);

            for edge in hm.get(&stop).unwrap_or(&vec![]) {
                if seen.contains(&edge.0) {
                    continue;
                }

                let mut busses = busses.clone();
                busses.insert(edge.1);
                queue.push_back((edge.0, busses));
            }
        }

        min_busses as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_num_buses_to_destination() {
        assert_eq!(Solution::num_buses_to_destination(vec_vec_i32![[1, 2, 7], [3, 6, 7]], 1, 6), 2);
    }

    #[test]
    fn test_num_buses_to_destination2() {
        assert_eq!(
            Solution::num_buses_to_destination(vec_vec_i32![[7, 12], [4, 5, 15], [6], [15, 19], [9, 12, 13]], 15, 12),
            -1
        );
    }
}
