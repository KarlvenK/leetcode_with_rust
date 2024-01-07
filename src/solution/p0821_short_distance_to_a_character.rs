use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        fn abs(x: i32) -> i32 {
            if x < 0 {
                return -x;
            }
            x
        }

        let n = s.len();
        let mut ans = vec![1234567890; n];
        let mut left = vec![1234567890; n];
        let mut right = vec![1234567890; n];

        for (i, &ch) in s.as_bytes().iter().enumerate() {
            let ch = ch as char;
            if ch == c {
                left[i] = i;
            } else {
                if i == 0 {
                    continue;
                }
                left[i] = left[i - 1];
            }
        }

        let mut i = n - 1;
        while i >= 0 {
            if (s.as_bytes()[i] as char) == c {
                right[i] = i;
            } else {
                if i != n - 1 {
                    right[i] = right[i + 1];
                }
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        for i in 0..n {
            if left[i] != 1234567890 {
                ans[i] = min(ans[i], abs(i as i32 - left[i] as i32));
            }
            if right[i] != 1234567890 {
                ans[i] = min(ans[i], abs(i as i32 - right[i] as i32));
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_821() {
        assert_eq!(
            Solution::shortest_to_char("loveleetcode".to_string(), 'e'),
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
        let mut s = "test".to_string();
    }
}
