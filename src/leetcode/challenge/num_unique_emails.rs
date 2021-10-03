use std::collections::HashSet;

// 929. Unique Email Addresses, Easy
// https://leetcode.com/problems/unique-email-addresses/
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = HashSet::new();

        for email in emails {
            let email: Vec<_> = email.split('@').collect();
            let mut vhost: String = email[0].to_string().replace(".", "");
            if vhost.contains('+') {
                vhost = vhost.split('+').next().unwrap().to_string();
            }

            let domain: String = email[1].to_string();

            set.insert(format!("{}@{}", vhost, domain));
        }

        set.len() as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_string;

    #[test]
    fn test_num_unique_emails() {
        assert_eq!(
            Solution::num_unique_emails(vec_string![
                "test.email+alex@leetcode.com",
                "test.e.mail+bob.cathy@leetcode.com",
                "testemail+david@lee.tcode.com"
            ]),
            2
        );
    }

    #[test]
    fn test_num_unique_emails2() {
        assert_eq!(Solution::num_unique_emails(vec_string!["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"]), 3);
    }

    #[test]
    fn test_num_unique_emails3() {
        assert_eq!(Solution::num_unique_emails(vec_string!["a@leetcode.com", "a@leetcode.com"]), 1);
    }

    #[test]
    fn test_num_unique_emails4() {
        assert_eq!(Solution::num_unique_emails(vec_string![]), 0);
    }
}
