use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
        let mut alpha_cnt: Vec<i32> = Vec::new();
        let mut num_cnt: Vec<i32> = Vec::new();
        let n = array.len();
        let mut map: HashMap<i32, i32> = HashMap::new(); //num - alpha
        map.insert(0, -1);
        alpha_cnt.resize(n + 1, 0);
        num_cnt.resize(n + 1, 0);

        let mut ans = 0;
        let (mut left, mut right) = (0, 0);

        for (i, str) in array.iter().enumerate() {
            num_cnt[i + 1] = num_cnt[i];
            alpha_cnt[i + 1] = alpha_cnt[i];
            match str.parse::<i32>() {
                Ok(_) => num_cnt[i + 1] += 1,
                Err(_) => alpha_cnt[i + 1] += 1,
            }
            let k = num_cnt[i + 1] - alpha_cnt[i + 1];
            match map.get(&k) {
                None => {
                    map.insert(k, i as i32);
                }
                Some(&idx) => {
                    if i as i32 - idx > ans {
                        ans = i as i32 - idx;
                        left = idx + 1;
                        right = i;
                    } else if i as i32 - idx == ans && idx + 1 < left {
                        left = idx + 1;
                        right = i;
                    }
                }
            }
        }
        if ans == 0 {
            return vec![];
        }
        array
            .into_iter()
            .enumerate()
            .filter(|(i, _)| *i >= left as usize && *i <= right)
            .map(|(_, s)| s)
            .collect()
    }
}
