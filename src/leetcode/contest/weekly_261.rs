use std::collections::HashMap;

// 2027. Minimum Moves to Convert String, Easy
// https://leetcode.com/problems/minimum-moves-to-convert-string/
impl Solution1 {
    pub fn minimum_moves(s: String) -> i32 {
        let mut arr: Vec<i32> = vec![];

        for ch in s.chars() {
            if ch == 'X' {
                arr.push(1);
            } else {
                arr.push(0);
            }
        }

        let mut moves = 0;
        let n = arr.len();
        let [mut l, mut r] = [0, 0];
        while r < n {
            if r - l < 2 {
                r += 1;
            } else {
                if arr[l] == 1 {
                    arr[l] = 0;
                    arr[l + 1] = 0;
                    arr[l + 2] = 0;
                    moves += 1;
                } else {
                    l += 1;
                }
            }

            if r == n && (arr[r - 1] == 1 || arr[r - 2] == 1 || arr[r - 3] == 1) {
                moves += 1;
            }
        }

        moves
    }
}

// 2028. Find Missing Observations, Medium
// https://leetcode.com/problems/find-missing-observations/
impl Solution2 {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        if rolls.len() == 0 || n == 0 {
            return vec![];
        }

        let m = rolls.len() as i32;

        let have_sum = rolls.iter().sum::<i32>();
        let mut missing_sum = (mean * (n + m)) - have_sum;

        let avg_missing = missing_sum / n;
        let mut action = true;
        if avg_missing > 0 && avg_missing <= 6 {
            let mut missing_rolls = vec![avg_missing; n as usize];
            missing_sum -= avg_missing * n;
            'outer: while missing_sum > 0 {
                for i in 0..n {
                    if missing_sum <= 0 || !action {
                        if !action {
                            return vec![];
                        }
                        break 'outer;
                    }
                    if missing_rolls[i as usize] < 6 {
                        let add = i32::min(6 - missing_rolls[i as usize], missing_sum);
                        missing_rolls[i as usize] += add;
                        missing_sum -= add;
                        action = true;
                    } else {
                        action = false;
                    }
                }
            }

            return missing_rolls;
        } else {
            return vec![];
        }
    }
}

struct Solution1 {}
struct Solution2 {}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_i32};

    #[test]
    fn test_minimum_moves() {
        assert_eq!(Solution1::minimum_moves("XXX".to_string()), 1);
    }

    #[test]
    fn test_minimum_moves2() {
        assert_eq!(Solution1::minimum_moves("XXOX".to_string()), 2);
    }

    #[test]
    fn test_minimum_moves3() {
        assert_eq!(Solution1::minimum_moves("OOOO".to_string()), 0);
    }

    #[test]
    fn test_missing_rolls() {
        assert_eq!(Solution2::missing_rolls(vec![3, 2, 4, 3], 4, 2), vec![6, 6]);
    }

    #[test]
    fn test_missing_rolls2() {
        assert_eq!(Solution2::missing_rolls(vec![1, 5, 6], 3, 4), vec![3, 2, 2, 2]);
    }

    #[test]
    fn test_missing_rolls3() {
        assert_eq!(Solution2::missing_rolls(vec![1, 2, 3, 4], 6, 4), vec![]);
    }

    #[test]
    fn test_missing_rolls4() {
        assert_eq!(Solution2::missing_rolls(vec![1], 3, 1), vec![5]);
    }

    #[test]
    fn test_missing_rolls5() {
        assert_eq!(Solution2::missing_rolls(vec![3, 5, 4], 5, 3), vec![6, 6, 6]);
    }

    #[test]
    fn test_missing_rolls6() {
        assert_eq!(Solution2::missing_rolls(vec![3, 5, 3], 5, 3), vec![]);
    }
}
