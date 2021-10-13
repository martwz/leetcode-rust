use std::collections::BinaryHeap;

// 253. Meeting Rooms II, Medium
// https://leetcode.com/problems/meeting-rooms-ii/
impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut rooms = 0;
        let mut heap = BinaryHeap::new();

        for interval in intervals.iter() {
            if heap.is_empty() {
                heap.push(interval[1]);
                rooms += 1;
            } else if heap.peek().unwrap() <= &interval[0] {
                heap.pop();
                heap.push(interval[1]);
            } else {
                heap.push(interval[1]);
                rooms += 1;
            }
        }

        rooms
    }
}

struct Solution {}
