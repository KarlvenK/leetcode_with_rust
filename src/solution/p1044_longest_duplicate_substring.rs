use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let mut l = 1;
        let mut r = s.len() - 1;
        let mut ans = "".to_string();
        while l <= r {
            let m = (r - l) / 2 + l;
            if let Some(x) = Solution::check(&s, m) {
                l = m + 1;
                ans = x;
            } else {
                r = m - 1;
            }
        }
        ans
    }

    fn check(s: &String, length: usize) -> Option<String> {
        let base = 37;
        let arr = s.as_bytes();
        let mut map = HashMap::<u64, usize>::new();
        let mut power = 1;
        let mut hash = 0;
        for i in 0..length {
            power *= base;
            hash = hash * base + (arr[i] as u64 - 97);
        }
        map.insert(hash, 0);

        for i in length..s.len() {
            hash = hash * base + arr[i] as u64 - 97;
            hash -= power * (arr[i - length] as u64 - 97);
            if map.contains_key(&hash) {
                let idx = *map.get(&hash).unwrap();
                return Some(s[idx..idx + length].to_string());
            }
            map.insert(hash, i - length + 1);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1044() {
        assert_eq!("ana", Solution::longest_dup_substring("banana".to_string()));
        assert_eq!("", Solution::longest_dup_substring("abcd".to_string()));
    }
}
