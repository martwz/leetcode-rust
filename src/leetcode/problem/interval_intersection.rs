// 986. Interval List Intersections, Medium
// https://leetcode.com/problems/interval-list-intersections/
impl Solution {
    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];

        let (mut i, mut j) = (0, 0);

        while i < first_list.len() && j < second_list.len() {
            let start = i32::max(first_list[i][0], second_list[j][0]);
            let end = i32::min(first_list[i][1], second_list[j][1]);

            if start <= end {
                ans.push(vec![start, end]);
            }

            if first_list[i][1] < second_list[j][1] {
                i += 1;
            } else {
                j += 1;
            }
        }

        ans
    }
}

struct Solution {}
