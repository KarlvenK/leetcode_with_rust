pub struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        Solution::fib(n - 1) + Solution::fib(n - 2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_509() {
        assert_eq!(Solution::fib(4), 3);
    }
}
