use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut map = HashMap::new();
        map.insert('2', "abc".to_string());
        map.insert('3', "def".to_string());
        map.insert('4', "ghi".to_string());
        map.insert('5', "jkl".to_string());
        map.insert('6', "mno".to_string());
        map.insert('7', "pqrs".to_string());
        map.insert('8', "tuv".to_string());
        map.insert('9', "wxyz".to_string());

        let mut ans = Vec::new();

        fn dfs(
            idx: usize,
            digits: &String,
            map: &HashMap<char, String>,
            temp: &mut String,
            ans: &mut Vec<String>,
        ) {
            if idx == digits.len() {
                ans.push(temp.clone());
            } else {
                let dig = digits.chars().nth(idx).unwrap();
                for c in map.get(&dig).unwrap().chars() {
                    temp.push(c);
                    dfs(idx + 1, digits, map, temp, ans);
                    temp.pop();
                }
            }
        }
        if digits.is_empty() {
            return ans;
        }
        dfs(0, &digits, &map, &mut String::new(), &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_17() {}
}
