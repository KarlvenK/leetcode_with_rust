pub struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut pars: Vec<char> = vec![' '; (n * 2) as usize];
        let mut ans: Vec<String> = vec![];
        Solution::dfs(&mut pars, &mut ans, 0, 0, 0);
        ans
    }
    fn dfs(pars: &mut Vec<char>, ans: &mut Vec<String>, curr: usize, cnt: i32, left_cnt: i32) {
        if curr == pars.len() {
            // println!("{}", pars.iter().collect::<String>());
            ans.push(pars.iter().collect());
            return;
        }
        if cnt > 0 {
            pars[curr] = ')';
            Solution::dfs(pars, ans, curr + 1, cnt - 1, left_cnt);
        }
        if left_cnt * 2 < pars.len() as i32 {
            pars[curr] = '(';
            Solution::dfs(pars, ans, curr + 1, cnt + 1, left_cnt + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        let mut ret = Solution::generate_parenthesis(3);
        ret.sort();
        let mut ans = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        ans.sort();
        assert_eq!(ans, ret);
    }
}
