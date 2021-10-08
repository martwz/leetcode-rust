use std::collections::{HashMap, HashSet};

// 1604. Alert Using Same Key-Card Three or More Times in a One Hour Period, Medium
// https://leetcode.com/problems/alert-using-same-key-card-three-or-more-times-in-a-one-hour-period/
impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let n = key_name.len();
        let mut time_map = HashMap::<String, Vec<i32>>::new();

        // HH:MM
        fn parse_time(time: &String) -> i32 {
            let chars = time.chars().collect::<Vec<char>>();
            ((chars[0].to_digit(10).unwrap() * 10 + chars[1].to_digit(10).unwrap()) * 60 + (chars[3].to_digit(10).unwrap() * 10 + chars[4].to_digit(10).unwrap())) as i32
        }

        let mut ans = HashSet::<String>::new();
        for i in 0..n {
            let name = &key_name[i];
            let time = parse_time(&key_time[i]);

            time_map.entry(name.clone()).or_insert(Vec::<i32>::new());
            time_map.get_mut(name).unwrap().push(time);
        }

        time_map.iter_mut().for_each(|(_, times)| {
            times.sort_unstable();
        });

        time_map.iter().for_each(|(name, times)| {
            let mut i = 0;
            while times.len() - i > 2 {
                if i32::abs(times[i] - times[i + 2]) <= 60 {
                    ans.insert(name.clone());
                }
                i += 1;
            }
        });

        let mut ans = ans.iter().cloned().collect::<Vec<String>>();
        ans.sort_unstable();
        ans
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn test_alert_names() {
        assert_eq!(
            Solution::alert_names(
                vec_string!["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"],
                vec_string!["10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"]
            ),
            vec_string!["daniel"]
        );
    }

    #[test]
    fn test_alert_names2() {
        assert_eq!(
            Solution::alert_names(
                vec_string!["alice", "alice", "alice", "bob", "bob", "bob", "bob"],
                vec_string!["12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"]
            ),
            vec_string!["bob"]
        );
    }

    #[test]
    fn test_alert_names3() {
        assert_eq!(
            Solution::alert_names(vec_string!["john", "john", "john"], vec_string!["23:58", "23:59", "00:01"]),
            vec_string![]
        );
    }

    #[test]
    fn test_alert_names4() {
        assert_eq!(
            Solution::alert_names(
                vec_string!["leslie", "leslie", "leslie", "clare", "clare", "clare", "clare"],
                vec_string!["13:00", "13:20", "14:00", "18:00", "18:51", "19:30", "19:49"]
            ),
            vec_string!["clare", "leslie"]
        );
    }

    #[test]
    fn test_alert_names5() {
        assert_eq!(
            Solution::alert_names(
                vec_string!["a", "a", "a", "a", "a", "a", "b", "b", "b", "b", "b"],
                vec_string!["23:27", "03:14", "12:57", "13:35", "13:18", "21:58", "22:39", "10:49", "19:37", "14:14", "10:41"]
            ),
            vec_string!["a"]
        );
    }

    #[test]
    fn test_alert_names6() {
        assert_eq!(
            Solution::alert_names(
                vec_string!["a", "a", "a", "a", "a", "b", "b", "b", "b", "b", "b"],
                vec_string!["23:20", "11:09", "23:30", "23:02", "15:28", "22:57", "23:40", "03:43", "21:55", "20:38", "00:19"]
            ),
            vec_string!["a"]
        );
    }
}
