pub struct Solution {}

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut has_upper = 0;
        let mut head_upper = false;
        let mut has_lower = 0;

        for (idx, ch) in word.chars().enumerate() {
            if idx == 0 {
                if ch.is_ascii_uppercase() {
                    head_upper = true;
                    has_upper += 1;
                } else {
                    has_lower += 1;
                }
                continue;
            }
            if ch.is_ascii_uppercase() {
                has_upper += 1;
            } else {
                has_lower += 1;
            }
            if has_lower > 0 && has_upper > 0 {
                if head_upper == false {
                    return false;
                } else {
                    if has_upper > 1 {
                        return false;
                    }
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_520() {
        assert_eq!(Solution::detect_capital_use("USA".into()), true);
        assert_eq!(Solution::detect_capital_use("Flag".into()), true);
        assert_eq!(Solution::detect_capital_use("g".into()), true);
        assert_eq!(Solution::detect_capital_use("FlaG".into()), false);
        assert_eq!(Solution::detect_capital_use("mL".into()), false);
    }
}
