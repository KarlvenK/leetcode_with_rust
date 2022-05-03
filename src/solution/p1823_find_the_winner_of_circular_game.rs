pub struct Solution {}
impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        fn dfs(n: i32, k: i32) -> i32 {
            if n == 1 {
                return 1;
            }
            let mut ans = dfs(n - 1, k) + k;
            ans = ans % n;
            if ans == 0 {
                return n;
            }
            return ans;
        }
        dfs(n, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1823() {
        assert_eq!(Solution::find_the_winner(5, 2), 3);
        assert_eq!(Solution::find_the_winner(6, 5), 1);
        assert_eq!(Solution::find_the_winner(8, 8), 4);
    }
}
