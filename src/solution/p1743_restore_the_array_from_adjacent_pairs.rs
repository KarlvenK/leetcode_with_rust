pub struct Solution {}

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        let n = adjacent_pairs.len() + 1;
        for pair in adjacent_pairs {
            let (key1, key2) = (pair[0], pair[1]);
            let entry_1 = map.entry(key1).or_insert(Vec::<i32>::new());
            entry_1.push(key2);
            let entry_2 = map.entry(key2).or_insert(Vec::<i32>::new());
            entry_2.push(key1);
        }

        let mut ans = Vec::<i32>::new();
        ans.resize(n, 0);
        for (key, value) in map.iter() {
            if value.len() == 1 {
                ans[0] = *key;
                break;
            }
        }
        ans[1] = *(*map.get(&ans[0]).unwrap()).get(0).unwrap();
        for i in 2..n {
            let pre = ans[i - 1];
            let adjs = map.get(&pre).unwrap();
            ans[i] = if ans[i - 2] == *adjs.get(0).unwrap() {
                *adjs.get(1).unwrap()
            } else {
                *adjs.get(0).unwrap()
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1743() {
        assert_eq!(
            Solution::restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]]),
            vec![1, 2, 3, 4]
        );
    }
}
