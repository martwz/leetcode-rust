use std::collections::VecDeque;

// 841. Keys and Rooms, Medium
// https://leetcode.com/problems/keys-and-rooms/
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut rooms = rooms;
        let n = rooms.len();

        let mut visited = vec![false; n];

        let mut q: VecDeque<i32> = VecDeque::new();
        q.push_back(0);
        visited[0] = true;

        while !q.is_empty() {
            let room = q.pop_front().unwrap() as usize;
            visited[room] = true;
            for key in &rooms[room] {
                q.push_back(*key);
            }
            rooms[room as usize] = vec![];
        }

        visited.iter().all(|v| *v)
    }
}

struct Solution {}
