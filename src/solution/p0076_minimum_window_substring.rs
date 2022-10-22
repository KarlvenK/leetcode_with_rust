use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut t_cnt = HashMap::new();
        let mut s_cnt = HashMap::new();
        let mut ans = "";
        let mut temp = 1234567890;
        let mut oks = 0;
        let (mut start, mut end) = (0, 0);

        for c in t.chars() {
            let en = t_cnt.entry(c).or_insert(0);
            *en += 1;
        }

        let (mut left, mut right) = (0, 0);

        let s_chars = s.chars().collect::<Vec<char>>();

        while right < s.len() {
            let c = s_chars[right];
            right += 1;
            if t_cnt.contains_key(&c) {
                let en = s_cnt.entry(c).or_insert(0);
                *en += 1;
                if en == t_cnt.get(&c).unwrap() {
                    oks += 1;
                }
            }
            while oks == t_cnt.len() {
                let steps = (right - left) as i32;
                if steps < temp {
                    temp = steps;
                    start = left;
                    end = right;
                }
                let d = s_chars[left];
                left += 1;
                if t_cnt.contains_key(&d) {
                    let en = s_cnt.get_mut(&d).unwrap();
                    if en == t_cnt.get(&d).unwrap() {
                        oks -= 1;
                    }
                    *en -= 1;
                }
            }
        }
        if temp == 1234567890 {
            return "".to_string();
        }
        s[start..end].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_76() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
    }
}
