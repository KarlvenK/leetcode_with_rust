use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let mut chs = s.chars().collect::<Vec<char>>();
            chs.sort();
            let ss = chs.iter().collect();
            let entry = map.entry(ss).or_insert(vec![]);
            entry.push(s);
        }
        let mut ans = Vec::new();
        for val in map.values() {
            ans.push(val.to_owned());
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_49() {
        assert_eq!(
            Solution::group_anagrams(
                vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                    .iter()
                    .map(|&x| x.to_string())
                    .collect()
            ),
            vec![vec!["eat", "tea", "ate"], vec!["bat"], vec!["tan", "nat"]]
                .iter()
                .map(|x| x.iter().map(|&x| x.to_string()).collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>()
        );
    }
}
