// https://leetcode.com/problems/expression-add-operators/
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut ans = vec![];

        fn force(num: &String, target: i32, prev_effect: Option<i32>, i: i32, equation: String, res: i32, ans: &mut Vec<String>) {
            if i == num.len() as i32 {
                if target == res {
                    ans.push(equation);
                }
            } else if i < num.len() as i32 {
                let digit = num[i as usize..(i + 1) as usize].parse::<i32>().unwrap();
                let digit_str = &digit.to_string();

                if i == 0 {
                    force(num, target, None, i + 1, digit.to_string(), digit, ans);
                } else {
                    force(num, target, Some(digit), i + 1, equation.clone() + "+" + digit_str, res + digit, ans);
                    force(num, target, Some(-digit), i + 1, equation.clone() + "-" + digit_str, res - digit, ans);
                    if prev_effect.is_some() {
                        force(
                            num,
                            target,
                            None,
                            i + 1,
                            equation.clone() + "*" + digit_str,
                            (res - prev_effect.unwrap()) + (prev_effect.unwrap() * digit),
                            ans,
                        );
                    } else {
                        force(num, target, None, i + 1, equation.clone() + "*" + digit_str, res * digit, ans);
                    }
                }
            }
        }

        force(&num, target, None, 0, "".to_string(), 0, &mut ans);

        return ans;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_operators() {
        assert_eq!(Solution::add_operators("123".to_string(), 6), vec!["1+2+3", "1*2*3"]);
    }

    #[test]
    fn test_add_operators2() {
        assert_eq!(Solution::add_operators("1".to_string(), 1), vec!["1"]);
    }

    #[test]
    fn test_add_operators3() {
        assert_eq!(Solution::add_operators("12".to_string(), 4), [] as [&str; 0]);
    }

    #[test]
    fn test_add_operators4() {
        assert_eq!(Solution::add_operators("232".to_string(), 8), vec!["2+3*2", "2*3+2"]);
    }
}
