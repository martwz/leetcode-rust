use std::collections::HashMap;

// 2022. Convert 1D Array Into 2D Array, Easy
// https://leetcode.com/problems/convert-1d-array-into-2d-array/
impl Solution1 {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        // check space
        if original.len() != m as usize * n as usize {
            return vec![];
        }

        let mut ans = vec![vec![0; n as usize]; m as usize];
        let mut idx = 0;
        for a in original {
            ans[idx / n as usize][idx % n as usize] = a;
            idx += 1;
        }

        ans
    }
}

// 2023. Number of Pairs of Strings With Concatenation Equal to Target, Medium
// https://leetcode.com/problems/number-of-pairs-of-strings-with-concatenation-equal-to-target/
impl Solution2 {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i != j {
                    if [nums[i].clone(), nums[j].clone()].concat() == target {
                        ans += 1;
                    }
                }
            }
        }

        ans as i32
    }
}

// 2024. Maximize the Confusion of an Exam, Medium
// https://leetcode.com/problems/maximize-the-confusion-of-an-exam/
impl Solution3 {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let nums: Vec<i32> = answer_key.chars().map(|c| if c == 'F' { 1 } else { 0 }).collect();

        let [mut l, mut r] = [0, 0];
        let mut uses = 0;
        let n = nums.len();
        let mut ans = 0;

        while r < n {
            if nums[r] == 0 {
                uses += 1;
            }
            while uses > k {
                if nums[l] == 0 {
                    uses -= 1;
                }
                l += 1;
            }

            ans = ans.max(r - l);
            r += 1;
        }

        r = 0;
        l = 0;
        uses = 0;

        while r < n {
            if nums[r] == 1 {
                uses += 1;
            }
            while uses > k {
                if nums[l] == 1 {
                    uses -= 1;
                }
                l += 1;
            }

            ans = ans.max(r - l);
            r += 1;
        }

        ans as i32 + 1
    }
}

// 2025. Maximum Number of Ways to Partition an Array, Hard
// https://leetcode.com/problems/maximum-number-of-ways-to-partition-an-array/
impl Solution4 {
    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let mut pre_calcs = Vec::<(i32, i32, usize)>::new();

        let mut part1 = 0;
        let mut part2 = nums.iter().sum::<i32>();
        for i in 0..nums.len() {
            part1 = part1 + nums[i];
            part2 = part2 - nums[i];

            pre_calcs.push((part1, part2, i));
        }

        let mut max = 0;
        for i in 0..nums.len() {
            let mut curr = 0;
            for pre_calc in pre_calcs.iter() {
                if i <= pre_calc.2 && pre_calc.0 - nums[i] + k == pre_calc.1 {
                    curr += 1;
                } else if i >= pre_calc.2 && pre_calc.0 == pre_calc.1 - nums[i] + k {
                    curr += 1;
                } else if pre_calc.0 == pre_calc.1 {
                    curr += 1;
                }
            }

            max = max.max(curr - 1);
        }

        max
    }
}

struct Solution1 {}
struct Solution2 {}
struct Solution3 {}
struct Solution4 {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_string, vec_vec_i32};

    #[test]
    fn test_construct2_d_array() {
        assert_eq!(Solution1::construct2_d_array(vec![1, 2, 3, 4], 2, 2), vec_vec_i32![[1, 2], [3, 4]]);
    }

    #[test]
    fn test_construct2_d_array2() {
        assert_eq!(Solution1::construct2_d_array(vec![1, 2, 3], 1, 3), vec_vec_i32![[1, 2, 3]]);
    }

    #[test]
    fn test_construct2_d_array3() {
        assert_eq!(Solution1::construct2_d_array(vec![1, 2], 1, 1), vec_vec_i32![]);
    }

    #[test]
    fn test_construct2_d_array4() {
        assert_eq!(Solution1::construct2_d_array(vec![3], 1, 2), vec_vec_i32![]);
    }

    #[test]
    fn test_num_of_pairs() {
        assert_eq!(Solution2::num_of_pairs(vec_string!["777", "7", "77", "77"], "7777".to_string()), 4);
    }

    #[test]
    fn test_num_of_pairs2() {
        assert_eq!(Solution2::num_of_pairs(vec_string!["123", "4", "12", "34"], "1234".to_string()), 2);
    }

    #[test]
    fn test_num_of_pairs3() {
        assert_eq!(Solution2::num_of_pairs(vec_string!["1", "1", "1"], "11".to_string()), 6);
    }

    #[test]
    fn test_max_consecutive_answers() {
        assert_eq!(Solution3::max_consecutive_answers("TTFF".to_string(), 2), 4);
    }

    #[test]
    fn test_max_consecutive_answers2() {
        assert_eq!(Solution3::max_consecutive_answers("TFFT".to_string(), 1), 3);
    }

    #[test]
    fn test_max_consecutive_answers3() {
        assert_eq!(Solution3::max_consecutive_answers("TTFTTFTT".to_string(), 1), 5);
    }

    #[test]
    fn test_max_consecutive_answers4() {
        assert_eq!(
            Solution3::max_consecutive_answers(
                "FTFFTFTFTTFTTFTTFFTTFFTTTTTFTTTFTFFTTFFFFFTTTTFTTTTTTTTTFTTFFTTFTFFTTTFFFFFTTTFFTTTTFTFTFFTTFTTTTTTF".to_string(),
                32
            ),
            85
        );
    }

    #[test]
    fn test_ways_to_partition() {
        assert_eq!(Solution4::ways_to_partition(vec![2, -1, 2], 3), 1);
    }

    #[test]
    fn test_ways_to_partition2() {
        assert_eq!(Solution4::ways_to_partition(vec![0, 0, 0], 1), 2);
    }

    #[test]
    fn test_ways_to_partition4() {
        assert_eq!(
            Solution4::ways_to_partition(
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30827, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                0
            ),
            33
        );
    }

    #[test]
    fn test_ways_to_partition5() {
        assert_eq!(
            Solution4::ways_to_partition(
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -93166, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 93166, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                69428
            ),
            37
        );
    }
}
