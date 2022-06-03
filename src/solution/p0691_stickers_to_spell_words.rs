use std::collections::HashMap;

pub struct Solution {}
fn min(a: i32, b: i32) -> i32 {
    if a > b {
        b
    } else {
        a
    }
}
impl Solution {
    pub fn min_stickers(mut stickers: Vec<String>, target: String) -> i32 {
        let m = target.len();
        let n: usize = 1 << m;
        let mut f = vec![-1; n];
        f[0] = 0;

        fn dp(mask: i32, stickers: &mut Vec<String>, target: &String, f: &mut Vec<i32>) -> i32 {
            if f[mask as usize] != -1 {
                return f[mask as usize];
            }
            f[mask as usize] = target.len() as i32 + 1;

            for i in 0..stickers.len() {
                let sticky = &stickers[i];
                let mut left = mask;
                let mut cnt = HashMap::new();
                for &ch in sticky.as_bytes() {
                    let e = cnt.entry(ch).or_insert(0);
                    *e += 1;
                }
                for (i, &ch) in target.as_bytes().into_iter().enumerate() {
                    if mask >> i & 1 == 1 && cnt.get(&ch) != None {
                        let e = cnt.get_mut(&ch).unwrap();
                        if e == &0 {
                            continue;
                        }
                        *e -= 1;
                        left ^= 1 << i;
                    }
                }
                if left < mask {
                    f[mask as usize] = min(f[mask as usize], dp(left, stickers, target, f) + 1);
                }
            }
            f[mask as usize]
        }
        let ans = dp((1 << m) - 1, &mut stickers, &target, &mut f);
        if ans <= (m as i32) {
            return ans;
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_691() {
        let mut a = vec!["with", "example", "science"];
        let a: Vec<String> = a.iter().map(|&x| x.to_string()).collect();
        assert_eq!(Solution::min_stickers(a, "thehat".to_string()), 3);
    }
}
