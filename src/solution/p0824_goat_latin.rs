use std::collections::HashSet;
use std::fmt::format;

pub struct Solution {}

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let set = HashSet::from(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
        let sentence = sentence.split_whitespace().collect::<Vec<_>>();
        sentence
            .iter()
            .enumerate()
            .map(|(k, &word)| {
                let c = word.chars().nth(0).unwrap();
                if set.contains(&c) {
                    format!("{}ma{}", word, "a".repeat(k + 1))
                } else {
                    format!("{}{}ma{}", &word[1..], c, "a".repeat(k + 1))
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_824() {
        assert_eq!(
            Solution::to_goat_latin("I speak Goat Latin".to_string()),
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string()
        );
    }
}
