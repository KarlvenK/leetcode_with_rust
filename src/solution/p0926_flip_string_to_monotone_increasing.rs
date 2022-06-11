pub struct Solution {}
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let n = s.len();
        let mut one_cnt = vec![0; n + 1];
        for (i, ch) in s.chars().enumerate() {
            if ch == '1' {
                one_cnt[i + 1] = one_cnt[i] + 1;
            } else {
                one_cnt[i + 1] = one_cnt[i];
            }
        }

        let mut ans = 1234567890;
        for i in 0..n + 1 {
            ans = Solution::min(ans, one_cnt[i] + (n - i) as i32 - (one_cnt[n] - one_cnt[i]));
        }
        ans
    }
    fn min(a: i32, b: i32) -> i32 {
        if a < b {
            a
        } else {
            b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_926() {
        let s = String::from("00110");
        assert_eq!(Solution::min_flips_mono_incr(s), 1);
        let s = String::from("010110");
        assert_eq!(Solution::min_flips_mono_incr(s), 2);
    }
}
