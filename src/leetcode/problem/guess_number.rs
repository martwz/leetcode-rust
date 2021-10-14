// 374. Guess Number Higher or Lower, Easy
// https://leetcode.com/problems/guess-number-higher-or-lower/
impl Solution {
    unsafe fn guess(n: i32) -> i32 {
        1
    }

    unsafe fn guess_umber(n: i32) -> i32 {
        let mut l = 0;
        let mut r = n;

        while l < r {
            let mid = l + (r - l) / 2;

            match Solution::guess(mid) {
                -1 => {
                    r = mid - 1;
                }
                1 => {
                    l = mid + 1;
                }
                0 => {
                    return mid;
                }
                _ => unreachable!(),
            }
        }

        l
    }
}

struct Solution {}
