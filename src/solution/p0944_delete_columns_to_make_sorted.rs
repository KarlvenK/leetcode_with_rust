pub struct Solution {}
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n: usize = strs[0].len();
        let mut ans = 0;

        for i in 0..n {
            for j in 1..strs.len() {
                if strs[j].get(i..=i).unwrap() < strs[j - 1].get(i..=i).unwrap() {
                    ans += 1;
                    break;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_944() {
        let strs = vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()];
        assert_eq!(Solution::min_deletion_size(strs), 1);
        let strs = vec!["a".to_string(), "b".to_string()];
        assert_eq!(Solution::min_deletion_size(strs), 0);
        let strs = vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()];
        assert_eq!(Solution::min_deletion_size(strs), 3);
    }
}
