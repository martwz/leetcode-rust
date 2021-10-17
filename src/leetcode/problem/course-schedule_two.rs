use std::collections::{HashMap, VecDeque};

// 210. Course Schedule II, Medium
// https://leetcode.com/problems/course-schedule-ii/
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut indegree = (0..num_courses).map(|f| 0).collect::<Vec<i32>>();

        let mut hm = HashMap::<i32, Vec<i32>>::new();
        for prereq in prerequisites.iter() {
            let [course, pre_course] = [prereq[0], prereq[1]];
            hm.entry(pre_course).and_modify(|f| f.push(course)).or_insert(vec![course]);
            indegree[course as usize] += 1;
        }

        let mut queue = VecDeque::new();
        for (i, course) in indegree.iter().enumerate() {
            if *course == 0 {
                queue.push_back(i as i32);
            }
        }

        let mut order: Vec<i32> = vec![];
        while !queue.is_empty() {
            let c = queue.pop_front().unwrap();
            order.push(c);

            for &v in hm.get(&c).unwrap_or(&vec![]) {
                indegree[v as usize] -= 1;
                if indegree[v as usize] == 0 {
                    queue.push_back(v);
                }
            }
        }

        if order.len() == num_courses as usize {
            order
        } else {
            vec![]
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_find_order() {
        assert_eq!(Solution::find_order(2, vec_vec_i32![[1, 0]]).len(), 2);
    }

    #[test]
    fn test_find_order2() {
        assert_eq!(Solution::find_order(4, vec_vec_i32![[1, 0], [2, 0], [3, 1], [3, 2]]).len(), 4);
    }

    #[test]
    fn test_find_order3() {
        assert_eq!(Solution::find_order(1, vec_vec_i32![]), [0]);
    }
}
