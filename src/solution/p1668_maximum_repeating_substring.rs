pub struct Solution {}

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut ans = 0;
        let mut i = 1;
        while i * word.len() <= sequence.len() {
            if sequence.contains(&word.repeat(i)) {
                ans = i as i32;
            }
            i += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1668() {
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ab".to_string()),
            2
        );
        assert_eq!(
            Solution::max_repeating("ababc".to_string(), "ba".to_string()),
            1
        )
    }
}
