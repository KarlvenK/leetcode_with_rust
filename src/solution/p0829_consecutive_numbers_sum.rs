pub struct Solution {}

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        // let (mut l, mut r) = (1, 1);
        // let mut sum = 1;
        // let mut ans = 0;
        // while l <= r && r <= n {
        //     if sum == n {
        //         ans += 1;
        //         r += 1;
        //         sum += r;
        //     } else {
        //         if sum < n {
        //             r += 1;
        //             sum += r;
        //         } else {
        //             sum -= l;
        //             l += 1;
        //         }
        //     }
        // }
        let mut ans = 0;

        for i in 1..=n {
            if i * (i + 1) > n * 2 {
                break;
            }
            if Solution::judge(n, i) {
                ans += 1;
            }
        }
        ans
    }
    fn judge(n: i32, k: i32) -> bool {
        if k & 1 == 1 {
            return (n % k) == 0;
        }
        return n % k != 0 && 2 * n % k == 0;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_829() {
        assert_eq!(Solution::consecutive_numbers_sum(65442209), 8);
        assert_eq!(Solution::consecutive_numbers_sum(5), 2);
        assert_eq!(Solution::consecutive_numbers_sum(9), 3);
        assert_eq!(Solution::consecutive_numbers_sum(15), 4);
    }
}
