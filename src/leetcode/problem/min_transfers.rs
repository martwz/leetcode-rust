use std::collections::HashMap;

// 465. Optimal Account Balancing, Hard
// https://leetcode.com/problems/optimal-account-balancing/
impl Solution {
    pub fn min_transfers(transactions: Vec<Vec<i32>>) -> i32 {
        let mut hm = HashMap::new();

        for transaction in transactions.iter() {
            let (from, to, amount) = (transaction[0], transaction[1], transaction[2]);
            hm.entry(from).and_modify(|d| *d -= amount).or_insert(-amount);
            hm.entry(to).and_modify(|d| *d += amount).or_insert(amount);
        }

        let mut debts = vec![];
        for v in hm.values() {
            debts.push(*v);
        }

        fn dfs(debts: &mut Vec<i32>, mut i: usize) -> i32 {
            while i < debts.len() && debts[i] == 0 {
                i += 1;
            }

            let mut ans = i32::MAX;
            let mut prev = 0;
            for j in i + 1..debts.len() {
                if debts[j] != prev && debts[j] * debts[i] < 0 {
                    debts[j] += debts[i];
                    ans = i32::min(ans, 1 + dfs(debts, i + 1));
                    debts[j] -= debts[i];
                    prev = debts[j];
                }
            }

            if ans < i32::MAX {
                ans
            } else {
                0
            }
        }

        dfs(&mut debts, 0)
    }
}

struct Solution {}
