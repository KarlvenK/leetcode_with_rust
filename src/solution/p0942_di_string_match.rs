pub struct Solution {}
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let n = (s.len() + 1) as i32;
        let (mut l, mut r) = (0, n - 1);
        let mut ans = Vec::with_capacity(n as usize);
        for &ch in s.as_bytes() {
            if ch as char == 'I' {
                ans.push(l);
                l += 1;
            } else {
                ans.push(r);
                r -= 1;
            }
        }
        ans.push(l);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_942() {
        assert_eq!(
            Solution::di_string_match("IDID".to_string()),
            vec![0, 4, 1, 3, 2]
        );
    }
}
