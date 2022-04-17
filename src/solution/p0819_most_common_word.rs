use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let paragraph = paragraph
            .to_lowercase()
            .replace("!", " ")
            .replace("?", " ")
            .replace("'", " ")
            .replace(",", " ")
            .replace(";", " ")
            .replace(".", " ");

        // println!("{}", paragraph);

        let words: Vec<&str> = paragraph.split(" ").collect();
        let banned_list = banned.iter().collect::<HashSet<&String>>();

        let mut map: HashMap<&str, i32> = HashMap::new();
        let mut ans = "";
        let mut cnt = 0;
        for word in words {
            if banned_list.contains(&word.to_string()) {
                continue;
            }
            if word == "" {
                continue;
            }
            let entry = map.entry(word).or_insert(0);
            *entry += 1;
            if *entry > cnt {
                cnt = *entry;
                ans = word;
            }
        }
        // println!("{:?}", map);
        ans.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_819() {
        assert_eq!(
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()]
            ),
            "ball"
        );
        assert_eq!(
            Solution::most_common_word(
                "Bob. hIt, baLl".to_string(),
                vec!["bob".to_string(), "hit".to_string()]
            ),
            "ball"
        );
    }
}
