pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::cmp::max;
        use std::collections::HashMap;

        let seq: Vec<char> = s.chars().collect();
        let mut map = HashMap::<char, i32>::new();
        let mut ans = 0;
        let n = s.len() as i32;
        let mut j = -1i32;

        for i in 0..n {
            if i != 0 {
                map.remove(&seq[(i - 1) as usize]);
            }
            while j + 1 < n {
                match map.get(&seq[(j + 1) as usize]) {
                    None => {
                        map.insert(seq[(j + 1) as usize], 1);
                        j += 1;
                    }
                    _ => {
                        break;
                    }
                }
            }
            ans = max(ans, j - i + 1);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
