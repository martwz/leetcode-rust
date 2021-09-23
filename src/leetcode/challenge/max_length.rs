// https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut dp = vec![[0; 26]];

        'outer: for str in arr.iter() {
            let mut bitmap = [0; 26];
            for ch in str.chars() {
                let i = ch as usize - 'a' as usize;
                bitmap[i] += 1;
                if bitmap[i] > 1 {
                    continue 'outer;
                }
            }

            let tmp_dp = dp.clone();
            'inner: for str_dp in tmp_dp.iter() {
                let mut tmp_bitmap = bitmap;
                for i in 0..26 {
                    tmp_bitmap[i] += str_dp[i];
                    if tmp_bitmap[i] > 1 {
                        continue 'inner;
                    }
                }

                dp.push(tmp_bitmap);
            }
        }

        dp.iter().map(|x| x.iter().sum::<i32>()).max().unwrap()
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_length() {
        assert_eq!(Solution::max_length(vec!["un".to_string(), "iq".to_string(), "ue".to_string()]), 4);
    }

    #[test]
    fn test_max_length2() {
        assert_eq!(Solution::max_length(vec!["cha".to_string(), "r".to_string(), "act".to_string(), "ers".to_string()]), 6);
    }

    #[test]
    fn test_max_length3() {
        assert_eq!(Solution::max_length(vec!["abcdefghijklmnopqrstuvwxyz".to_string()]), 26);
    }

    #[test]
    fn test_max_length4() {
        assert_eq!(Solution::max_length(vec!["yy".to_string(), "bkhwmpbiisbldzknpm".to_string()]), 0);
    }

    #[test]
    fn test_max_length5() {
        assert_eq!(
            Solution::max_length(vec!["a".to_string(), "abc".to_string(), "d".to_string(), "de".to_string(), "def".to_string()]),
            6
        );
    }
}
