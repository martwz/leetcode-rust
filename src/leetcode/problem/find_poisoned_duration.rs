// https://leetcode.com/problems/teemo-attacking/
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut sum = 0;

        let mut tick_off = 0;
        time_series.iter().for_each(|&t| {
            if t > tick_off {
                sum += duration;
            } else {
                sum += duration + (t - tick_off);
            }
            tick_off = duration + t;
        });
        return sum;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_poisoned_duration() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
    }

    #[test]
    fn test_find_poisoned_duration2() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
    }

    #[test]
    fn test_find_poisoned_duration3() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 3), 4);
    }
}
