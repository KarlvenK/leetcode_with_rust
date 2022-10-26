pub struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ret = vec![];
        let max_len;
        if word1.len() > word2.len() {
            max_len = word1.len();
        } else {
            max_len = word2.len();
        }

        for i in 0..max_len {
            if i < word1.len() {
                ret.push(word1.chars().nth(i).unwrap());
            }
            if i < word2.len() {
                ret.push(word2.chars().nth(i).unwrap());
            }
        }
        ret.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1768() {
        assert_eq!(
            Solution::merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        );
    }
}
