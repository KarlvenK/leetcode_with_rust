pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        for email in emails {
            let mut mail_name = String::new();
            let v: Vec<_> = email.splitn(2, '@').collect();
            let local_name = if v[0].contains('+') {
                let ret: Vec<_> = v[0].splitn(2, '+').collect();
                ret[0]
            } else {
                v[0]
            };
            mail_name.push_str(local_name);
            mail_name.push('@');
            mail_name.retain(|ch| ch != '.');
            mail_name.push_str(v[1]);
            set.insert(mail_name);
        }
        set.len() as i32
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_929() {
        let emails = vec![
            "test.email+alex@leetcode.com",
            "test.e.mail+bob.cathy@leetcode.com",
            "testemail+david@lee.tcode.com",
        ];
        let emails: Vec<String> = emails.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::num_unique_emails(emails), 2);
    }
}
