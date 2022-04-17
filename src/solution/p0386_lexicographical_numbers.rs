pub struct Solution {}

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        // let mut max_len = 5;
        fn dfs(ans: &mut Vec<i32>, num: i32, n: i32) {
            if num == 0 {
                for i in 1..=9 {
                    if i <= n {
                        ans.push(i);
                        dfs(ans, i, n);
                    }
                }
            } else {
                for i in 0..=9 {
                    if num * 10 + i <= n {
                        ans.push(num * 10 + i);
                        dfs(ans, num * 10 + i, n);
                    } else {
                        break;
                    }
                }
            }
        }
        dfs(&mut ans, 0, n);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_386() {
        assert_eq!(Solution::lexical_order(2), vec![1, 2]);
        assert_eq!(
            Solution::lexical_order(13),
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }
}
