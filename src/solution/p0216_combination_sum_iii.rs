pub struct Solution {}
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::<Vec<i32>>::new();
        let mut temp = Vec::<i32>::new();
        fn dfs(idx: i32, k: i32, n: i32, sum: i32, ans: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>) {
            if sum > n {
                return;
            }
            if idx == k {
                if sum == n {
                    ans.push(temp.clone());
                    return;
                }
                return;
            }
            let mut start: i32;
            match temp.last() {
                Some(&x) => {
                    start = x + 1;
                }
                None => {
                    start = 1;
                }
            }
            for x in start..=9 {
                temp.push(x);
                dfs(idx + 1, k, n, sum + x, ans, temp);
                temp.pop();
            }
        }
        dfs(0, k, n, 0, &mut ans, &mut temp);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_216() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]]);
    }
}
