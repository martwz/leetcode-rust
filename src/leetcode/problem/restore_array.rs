use std::collections::{HashMap, HashSet};

// 1743. Restore the Array From Adjacent Pairs, Medium
// https://leetcode.com/problems/restore-the-array-from-adjacent-pairs/
impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        for pair in adjacent_pairs.iter() {
            let (u, v) = (pair[0], pair[1]);
            graph.entry(u).and_modify(|e| e.push(v)).or_insert(vec![v]);
            graph.entry(v).and_modify(|e| e.push(u)).or_insert(vec![u]);
        }

        let n = adjacent_pairs.len() + 1;
        let mut ans = vec![];

        let mut head_cands = graph.iter().map(|(k, v)| (*k, v.len())).collect::<Vec<(i32, usize)>>();
        head_cands.sort_by_key(|(_, v)| *v);

        let mut head = head_cands.first().unwrap().0;
        let mut seen = HashSet::<i32>::new();

        while ans.len() < n {
            seen.insert(head);
            ans.push(head);
            
            let cands = graph.get(&head).unwrap();
            for cand in cands.iter() {
                if !seen.contains(&cand) {
                    seen.insert(*cand);
                    head = *cand;
                    break;
                }
            }
        }

        ans
    }
}

struct Solution {}
