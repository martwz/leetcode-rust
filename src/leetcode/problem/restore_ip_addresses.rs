// 93. Restore IP Addresses, Medium
// https://leetcode.com/problems/restore-ip-addresses/
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let digits = s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

        fn helper(blocks: Vec<Vec<u32>>, digits: &Vec<u32>, i: usize, ans: &mut Vec<String>) {
            if blocks.len() > 4 || i > digits.len() {
                
            } else if blocks.len() == 4 && i == digits.len() {
                let mut s = String::new();
                for block in blocks {
                    for d in block {
                        s.push_str(&d.to_string());
                    }
                    s.push('.');
                }
                s.pop();
                ans.push(s);
            } else {
                let mut block = vec![];
                let mut num = 0;
                for j in i..usize::min(digits.len(), i + 3) {
                    block.push(digits[j]);
                    num = num * 10 + digits[j];
                    if num > 255 || (block.first().unwrap() == &0 && block.len() > 1) {
                        break;
                    }

                    let mut b = blocks.clone();
                    b.push(block.clone());
                    helper(b, digits, j + 1, ans);
                }
            }
        }

        let mut ans: Vec<String> = vec![];
        helper(vec![], &digits, 0, &mut ans);
        ans
    }
}

struct Solution {}
