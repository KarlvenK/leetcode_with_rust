pub struct Solution{}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
            for i in 0..nums.len() {
                match map.get(&(target - nums[i])) {
                    None => {
                        map.insert(nums[i], i);
                    }
                    Some(ans) => {
                        return vec![*ans as i32, i as i32];
                    }
                }
            }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}