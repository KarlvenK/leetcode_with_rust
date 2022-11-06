pub struct Solution {}

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut ans = vec![];
        let mut pre = ' ';
        for ch in command.chars() {
            if ch == 'G' {
                ans.push('G');
            }
            if ch == ')' {
                if pre == '(' {
                    ans.push('o');
                } else {
                    ans.push('a');
                    ans.push('l');
                }
            }
            pre = ch;
        }
        ans.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1678() {
        assert_eq!(
            Solution::interpret("G()(al)".to_string()),
            "Goal".to_string()
        );
    }
}
