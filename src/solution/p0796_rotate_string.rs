pub struct Solution {}

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        return s.repeat(2).contains(&goal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_796() {
        assert_eq!(
            Solution::rotate_string("abcde".to_string(), "cdeab".to_string()),
            true
        );
        assert_eq!(
            Solution::rotate_string("abc".to_string(), "bcd".to_string()),
            false
        );
    }
}
