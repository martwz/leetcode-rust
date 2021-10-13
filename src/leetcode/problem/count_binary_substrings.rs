// 696. Count Binary Substrings, Easy
// https://leetcode.com/problems/count-binary-substrings/
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut chars = s.chars().collect::<Vec<char>>();
        chars.push('#');
        let n = s.len();
        if n == 0 {
            return 0;
        }

        let mut prev = chars[0];
        let mut count = 1;

        let mut counts = vec![];
        for i in 1..n {
            if prev == chars[i] {
                count += 1;
            } else {
                counts.push(count);
                prev = chars[i];
                count = 1;
            }
        }

        let mut substrings_count = 0;
        for i in 0..counts.len() - 1 {
            substrings_count += i32::min(counts[i], counts[i - 1]);
        }

        substrings_count
    }
}

struct Solution {}
