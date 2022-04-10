pub struct Solution {}

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        match n {
            0 => 1,
            1 => 10,
            _ => {
                let solve = |n| -> i32 {
                    let (mut ans, mut t) = (10, 9);
                    for i in 0..n - 1 {
                        t *= 9 - i;
                        ans += t;
                    }
                    ans
                };
                solve(n)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_357() {
        assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
        assert_eq!(Solution::count_numbers_with_unique_digits(0), 1);
    }
}
