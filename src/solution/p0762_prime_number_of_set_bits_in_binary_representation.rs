pub struct Solution {}

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut ans = 0;
        let get_cnt = |x: i32| {
            let (mut ret, mut v) = (0, x);
            while v != 0 {
                ret += 1;
                v -= v & (-v);
            }
            ret
        };
        let is_prime = |x: i32| {
            if x == 1 {
                return false;
            }
            if x == 2 {
                return true;
            }
            for i in 2..x {
                if i * i > x {
                    break;
                }
                if x % i == 0 {
                    return false;
                }
            }
            true
        };
        for num in left..right + 1 {
            let cnt = get_cnt(num);
            if is_prime(cnt) {
                ans += 1;
            }
        }
        ans
    }

    pub fn try_new(left: i32, right: i32) -> i32 {
        (left..=right).fold(0, |mut ret, i| {
            let (mut cnt, mut j) = (0, i);
            while j > 0 {
                cnt += &j & 1;
                j >>= 1;
            }
            if [2, 3, 5, 7, 11, 13, 17, 19].contains(&cnt) {
                ret += 1;
            }
            ret
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[test]
    fn test_1() {
        for i in 0..20 {
            let (mut left, mut right) = (
                rand::thread_rng().gen_range(0..100000),
                rand::thread_rng().gen_range(0..100000),
            );
            if left > right {
                (left, right) = (right, left);
            }
            assert_eq!(
                Solution::try_new(left, right),
                Solution::count_prime_set_bits(left, right)
            )
        }
    }
}
