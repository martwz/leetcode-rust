use std::collections::{HashMap, VecDeque};

// 207. Course Schedule, Medium
// https://leetcode.com/problems/course-schedule/
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
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

        let mut count = 0;
        while !queue.is_empty() {
            let c = queue.pop_front().unwrap();
            count += 1;

            for &v in hm.get(&c).unwrap_or(&vec![]) {
                indegree[v as usize] -= 1;
                if indegree[v as usize] == 0 {
                    queue.push_back(v);
                }
            }
        }

        count == num_courses
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec_i32;

    #[test]
    fn test_can_finish() {
        assert_eq!(Solution::can_finish(2, vec_vec_i32![[1, 0]]), true);
    }

    #[test]
    fn test_can_finish2() {
        assert_eq!(Solution::can_finish(2, vec_vec_i32![[1, 0], [0, 1]]), false);
    }

    #[test]
    fn test_can_finish3() {
        assert_eq!(
            Solution::can_finish(20, vec_vec_i32![[0, 10], [3, 18], [5, 5], [6, 11], [11, 14], [13, 1], [15, 1], [17, 4]]),
            false
        );
    }
}
