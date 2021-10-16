use std::collections::{HashMap, VecDeque};

// 5885. Minimum Number of Moves to Seat Everyone, Easy
// https://leetcode.com/problems/minimum-number-of-moves-to-seat-everyone/
impl Solution1 {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();

        let mut moves = 0;
        for i in 0..seats.len() {
            moves += i32::abs(seats[i] - students[i]);
        }

        moves
    }
}

// 5886. Remove Colored Pieces if Both Neighbors are the Same Color, Medium
// https://leetcode.com/problems/remove-colored-pieces-if-both-neighbors-are-the-same-color/
impl Solution2 {
    pub fn winner_of_game(colors: String) -> bool {
        let mut count_aaa = 0;
        let mut count_bbb = 0;

        let chars = colors.chars().collect::<Vec<char>>();
        let mut i = 1;
        let mut prev = chars[0];
        for j in 1..chars.len() {
            if chars[j] == prev {
                i += 1;
                if i == 3 && prev == 'A' {
                    count_aaa += 1;
                    i = 2;
                } else if i == 3 && prev == 'B' {
                    count_bbb += 1;
                    i = 2;
                }
            } else {
                prev = chars[j];
                i = 1;
            }
        }

        count_aaa > count_bbb
    }
}

// 5888. The Time When the Network Becomes Idle, Medium
// https://leetcode.com/problems/the-time-when-the-network-becomes-idle/
impl Solution3 {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let n = patience.len();
        let mut dist = vec![std::i32::MAX; n];
        let mut queue = VecDeque::new();
        let mut visited = vec![false; n];

        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n {
            graph.insert(i, vec![]);
        }
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph.get_mut(&u).unwrap().push(v);
            graph.get_mut(&v).unwrap().push(u);
        }

        dist[0] = 0;
        queue.push_back(0);

        while !queue.is_empty() {
            let u = queue.pop_front().unwrap();
            visited[u] = true;

            for &v in graph[&u].iter() {
                let v = v as usize;
                if dist[v] > dist[u] + patience[u] {
                    dist[v] = dist[u] + 1;
                    if !visited[v] {
                        queue.push_back(v);
                    }
                }
            }
        }

        let dist = dist.iter().map(|&f| f as f32 * 2.0).collect::<Vec<f32>>();
        let patience = patience.iter().map(|&f| f as f32).collect::<Vec<f32>>();

        let mut ans = 0.0;
        for i in 1..n {
            let left = f32::ceil(dist[i] / patience[i]) - 1.0;
            let remain_start = dist[i] - left * patience[i];
            ans = f32::max(ans, dist[i] - remain_start + dist[i]);
        }

        ans as i32 + 1
    }
}

struct Solution1 {}
struct Solution2 {}
struct Solution3 {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_i32};

    #[test]
    fn test_min_moves_to_seat() {
        assert_eq!(Solution1::min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]), 4);
    }

    #[test]
    fn test_min_moves_to_seat2() {
        assert_eq!(Solution1::min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]), 7);
    }

    #[test]
    fn test_min_moves_to_seat3() {
        assert_eq!(Solution1::min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6]), 4);
    }

    #[test]
    fn test_winner_of_game() {
        assert_eq!(Solution2::winner_of_game("AAABABB".to_string()), true);
    }

    #[test]
    fn test_winner_of_game2() {
        assert_eq!(Solution2::winner_of_game("AA".to_string()), false);
    }

    #[test]
    fn test_winner_of_game3() {
        assert_eq!(Solution2::winner_of_game("ABBBBBBBAAA".to_string()), false);
    }

    #[test]
    fn test_winner_of_game4() {
        assert_eq!(Solution2::winner_of_game("BBBAAAABB".to_string()), true);
    }

    #[test]
    fn test_network_becomes_idle() {
        assert_eq!(Solution3::network_becomes_idle(vec_vec_i32![[0, 1], [1, 2]], vec![0, 2, 1]), 8);
    }

    #[test]
    fn test_network_becomes_idle2() {
        assert_eq!(Solution3::network_becomes_idle(vec_vec_i32![[0, 1], [0, 2], [1, 2]], vec![0, 10, 10]), 3);
    }

    #[test]
    fn test_network_becomes_idle3() {
        assert_eq!(
            Solution3::network_becomes_idle(
                vec_vec_i32![
                    [3, 8],
                    [4, 13],
                    [0, 7],
                    [0, 4],
                    [1, 8],
                    [14, 1],
                    [7, 2],
                    [13, 10],
                    [9, 11],
                    [12, 14],
                    [0, 6],
                    [2, 12],
                    [11, 5],
                    [6, 9],
                    [10, 3]
                ],
                vec![0, 3, 2, 1, 5, 1, 5, 5, 3, 1, 2, 2, 2, 2, 1]
            ),
            20
        );
    }
}
