use std::collections::{BinaryHeap, HashMap};

// 1086. High Five, Easy
// https://leetcode.com/problems/high-five/
impl Solution {
    pub fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // items[i] = [IDi, scorei]
        let mut five_map = HashMap::<i32, BinaryHeap<i32>>::new();

        let mut ans: Vec<Vec<i32>> = vec![];
        for item in items.iter() {
            let [id, score] = [item[0], item[1]];
            if five_map.contains_key(&id) {
                five_map.entry(id).and_modify(|h| h.push(-score));
            } else {
                five_map.insert(id, BinaryHeap::from(vec![-score]));
            }

            if five_map.get_mut(&id).unwrap().len() > 5 {
                five_map.get_mut(&id).unwrap().pop();
            }
        }

        five_map.iter().for_each(|(id, heap)| {
            let sum = -heap.iter().sum::<i32>();
            ans.push(vec![*id, sum / heap.len() as i32])
        });

        ans.sort_unstable_by_key(|f| f[0]);

        ans
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_high_five() {
        assert_eq!(
            Solution::high_five(vec_vec_i32![
                [1, 91],
                [1, 92],
                [2, 93],
                [2, 97],
                [1, 60],
                [2, 77],
                [1, 65],
                [1, 87],
                [1, 100],
                [2, 100],
                [2, 76]
            ]),
            vec_vec_i32![[1, 87], [2, 88]]
        );
    }

    #[test]
    fn test_high_five2() {
        assert_eq!(
            Solution::high_five(vec_vec_i32![
                [1, 100],
                [7, 100],
                [1, 100],
                [7, 100],
                [1, 100],
                [7, 100],
                [1, 100],
                [7, 100],
                [1, 100],
                [7, 100]
            ]),
            vec_vec_i32![[1, 100], [7, 100]]
        );
    }
}
