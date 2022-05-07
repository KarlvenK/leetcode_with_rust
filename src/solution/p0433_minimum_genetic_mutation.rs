use std::cmp::min;
use std::collections::{HashMap, VecDeque};
pub struct Solution {}

impl Solution {
    pub fn min_mutation(start: String, end: String, mut bank: Vec<String>) -> i32 {
        let mut map = HashMap::new();
        let mut cnt = 0;
        bank.push(start.clone());

        fn check(s1: &str, s2: &str) -> bool {
            if s1.len() != s2.len() {
                return false;
            }
            let mut cnt = 0;

            let a = s1.as_bytes();
            let b = s2.as_bytes();

            for i in 0..s1.len() {
                if a[i] != b[i] {
                    cnt += 1;
                    if cnt > 1 {
                        return false;
                    }
                }
            }
            if cnt == 1 {
                return true;
            }
            false
        }
        for s in bank.iter() {
            map.entry(s).or_insert(cnt);
            cnt += 1;
        }
        if None == map.get(&end) {
            return -1;
        }

        let mut graph = vec![Vec::<usize>::new(); cnt];

        for i in 0..bank.len() - 1 {
            for j in i + 1..bank.len() {
                if check(&bank[i], &bank[j]) {
                    let &u = map.get(&bank[i]).unwrap();
                    let &v = map.get(&bank[j]).unwrap();
                    graph[u].push(v);
                    graph[v].push(u);
                }
            }
        }

        let mut que = VecDeque::new();
        que.push_back(*map.get(&start).unwrap());

        let mut dist = vec![1234567890; cnt];
        dist[*map.get(&start).unwrap()] = 0;
        while !que.is_empty() {
            let &u = que.front().unwrap();
            que.pop_front();
            for &v in &graph[u] {
                if dist[v] > dist[u] + 1 {
                    dist[v] = dist[u] + 1;
                    que.push_back(v);
                }
            }
        }
        if dist[*map.get(&end).unwrap()] == 1234567890 {
            return -1;
        }
        dist[*map.get(&end).unwrap()]
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_433() {
        let start = "AACCGGTT".to_string();
        let end = "AACCGGTA".to_string();
        assert_eq!(Solution::min_mutation(start, end, vec![]), -1);

        let start = "AACCGGTT".to_string();
        let end = "AACCGGTA".to_string();
        let bank = vec!["AACCGGTA".to_string()];
        assert_eq!(Solution::min_mutation(start, end, bank), 1);

        let start = "AACCGGTT".to_string();
        let end = "AAACGGTA".to_string();
        let bank = vec![
            "AACCGGTA".to_string(),
            "AACCGCTA".to_string(),
            "AAACGGTA".to_string(),
        ];
        assert_eq!(Solution::min_mutation(start, end, bank), 2);

        let start = "AAAAACCC".to_string();
        let end = "AACCCCCC".to_string();
        let bank = vec![
            "AAAACCCC".to_string(),
            "AAACCCCC".to_string(),
            "AACCCCCC".to_string(),
        ];
        assert_eq!(Solution::min_mutation(start, end, bank), 3);
    }
}
