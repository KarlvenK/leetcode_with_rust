pub struct Solution {}

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut pre_sum = vec![];
        pre_sum.resize(blocks.len(),0i32);
        for (i,&ch) in blocks.as_bytes().iter().enumerate() {
            if i > 0 {
                pre_sum[i] = pre_sum[i - 1]
            }
            if ch == b'B' {
                pre_sum[i] += 1;
            }
        }

        let mut ans = 1234567890;
        for i in 0..=(blocks.len() - k as usize) {
            let t = match i {
                0 => pre_sum[k as usize - 1],
                _ => pre_sum[i + k as usize - 1] - pre_sum[i - 1]
            };
            if ans > (k - t) {
                ans = k -t;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2379() {
        let s = "WBBWWBBWBW".to_string();
        assert_eq!(Solution::minimum_recolors(s, 7), 3);
        let s = "BBBBBBBWBW".to_string();
        assert_eq!(Solution::minimum_recolors(s, 2), 0);
    }
}