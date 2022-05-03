pub struct Solution {}
impl Solution {
    pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
        logs.sort_by_key(|log| {
            let mut parts = log.splitn(2, ' ');
            let one = parts.next().unwrap();
            let two = parts.next().unwrap();
            if two.chars().next().unwrap().is_alphabetic() {
                (0, two.to_string(), one.to_string())
            } else {
                (1, "".to_string(), "".to_string())
            }
        });
        logs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_937() {
        let logs: Vec<String> = vec![
            "dig1 8 1 5 1",
            "let1 art can",
            "dig2 3 6",
            "let2 own kit dig",
            "let3 art zero",
        ]
        .into_iter()
        .map(|a| a.to_string())
        .collect();
        let ans: Vec<String> = vec![
            "let1 art can",
            "let3 art zero",
            "let2 own kit dig",
            "dig1 8 1 5 1",
            "dig2 3 6",
        ]
        .into_iter()
        .map(|a| a.to_string())
        .collect();
        assert_eq!(Solution::reorder_log_files(logs), ans);

        let logs: Vec<String> = vec![
            "a1 9 2 3 1",
            "g1 act car",
            "zo4 4 7",
            "ab1 off key dog",
            "a8 act zoo",
        ]
        .into_iter()
        .map(|a| a.to_string())
        .collect();
        let ans: Vec<String> = vec![
            "g1 act car",
            "a8 act zoo",
            "ab1 off key dog",
            "a1 9 2 3 1",
            "zo4 4 7",
        ]
        .into_iter()
        .map(|a| a.to_string())
        .collect();
        assert_eq!(Solution::reorder_log_files(logs), ans);
    }
}
