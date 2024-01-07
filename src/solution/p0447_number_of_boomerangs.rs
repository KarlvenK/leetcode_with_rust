use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for (i, p) in points.iter().enumerate() {
            let mut map = HashMap::<i32, i32>::new();
            for (j, q) in points.iter().enumerate() {
                if i == j {
                    continue;
                }
                let entry = map.entry(Solution::dist(p, q)).or_insert(0);
                *entry += 1;
            }
            for (_, &v) in map.iter() {
                ans += v * (v - 1);
            }
        }
        ans
    }
    fn dist(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
        (a[0] - b[0]) * (a[0] - b[0]) + (a[1] - b[1]) * (a[1] - b[1])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_447() {
        assert_eq!(
            2,
            Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]])
        )
    }
}
