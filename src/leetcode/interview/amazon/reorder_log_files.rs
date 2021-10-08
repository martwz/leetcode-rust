// 937. Reorder Data in Log Files, Easy
// https://leetcode.com/problems/reorder-data-in-log-files/
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut letter_logs = vec![];
        let mut digit_logs = vec![];

        for log in logs {
            let chunks = log.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            let id = chunks[0].clone();
            let msg = &chunks[1..];
            let kind = {
                if msg[0].chars().next().unwrap().is_alphabetic() {
                    0 // letter
                } else {
                    1 // digit
                }
            };

            if kind == 0 {
                letter_logs.push((id, msg.join(" ")));
            } else {
                digit_logs.push(log);
            }
        }

        letter_logs.sort_by(|a, b| {
            let (id_a, msg_a) = a;
            let (id_b, msg_b) = b;
            msg_a.cmp(&msg_b).then_with(|| id_a.cmp(&id_b))
        });

        letter_logs.into_iter().map(|(id, msg)| format!("{} {}", id, msg)).into_iter().chain(digit_logs).collect()
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn test_reorder_log_files() {
        assert_eq!(
            Solution::reorder_log_files(vec_string!["dig1 8 1 5 1", "let1 art can", "dig2 3 6", "let2 own kit dig", "let3 art zero"]),
            vec_string!["let1 art can", "let3 art zero", "let2 own kit dig", "dig1 8 1 5 1", "dig2 3 6"]
        );
    }

    #[test]
    fn test_reorder_log_files2() {
        assert_eq!(
            Solution::reorder_log_files(vec_string!["a1 9 2 3 1", "g1 act car", "zo4 4 7", "ab1 off key dog", "a8 act zoo"]),
            vec_string!["g1 act car", "a8 act zoo", "ab1 off key dog", "a1 9 2 3 1", "zo4 4 7"]
        );
    }
}
