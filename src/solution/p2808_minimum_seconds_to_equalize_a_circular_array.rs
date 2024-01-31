use std::{cmp::max, cmp::min, collections::HashMap};
pub struct Solution {}

/*
由于nums数组最后都会变成同一个数字，我们假设是x，那么我们可以枚举这个数字x
对于一堆下标 (i, j), i < j 且 (i，j) 范围内不含x，(i, j) 全部变成x，需要 (j - i) / 2 向下取整秒，即两端同时向内收缩
还有一个特殊情况，是 x 第一次出现的坐标l和最后一次出现的坐标r 所含区间外的区域，根据题意，这个数组其实是类似环状的
这部分区域要长度是 n - (r -l) = l - r + n
 */
impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let mut pos_map: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &x) in nums.iter().enumerate() {
            let entry = pos_map.entry(x).or_insert(Vec::new());
            entry.push(i);
        }
        let mut ans = nums.len();
        let n = ans;
        for (_, pos) in pos_map.iter() {
            let mut loc_max = pos[0] + n - pos.last().unwrap();
            for (i, _) in pos.iter().enumerate().skip(1) {
                loc_max = max(loc_max, pos[i] - pos[i - 1]);
            }
            ans = min(ans, loc_max / 2);
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2808() {
        assert_eq!(Solution::minimum_seconds(vec![1, 2, 1, 2]), 1);
        assert_eq!(Solution::minimum_seconds(vec![2, 1, 3, 3, 2]), 2);
    }
}
