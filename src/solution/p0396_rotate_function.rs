use std::cmp::max;

pub struct Solution {}

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut pre = 0;
        let mut ans;
        let n = nums.len();

        for i in 0..n {
            pre += nums[i] * (i as i32);
        }
        ans = pre;
        for i in 1..n {
            pre += sum - (n as i32) * nums[n - i];
            ans = max(ans, pre);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_396() {
        assert_eq!(
            Solution::max_rotate_function(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            330
        );
        assert_eq!(Solution::max_rotate_function(vec![4, 3, 2, 6]), 26);
        assert_eq!(Solution::max_rotate_function(vec![100]), 0);
    }
}
