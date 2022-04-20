pub struct Solution {}

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut stk: Vec<&str> = vec![];
        let mut sum = 0;
        let mut ans = 0;
        for row in input.split('\n') {
            let name = row.trim_start_matches(|c| c == '\t');
            let depth = row.len() - name.len();
            while depth < stk.len() {
                sum -= stk.pop().unwrap().len();
            }
            sum += name.len();
            if name.contains('.') && sum + stk.len() > ans {
                ans = sum + stk.len();
            }
            stk.push(name);
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_388() {
        assert_eq!(Solution::length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".to_string()),
        32);
    }
}
