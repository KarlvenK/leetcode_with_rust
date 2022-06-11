pub struct Solution {}

use std::cmp::max;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let n = nums.len();
        let sum = nums.iter().sum::<i32>();
        let target = sum - x;
        let (mut left, mut right) = (0_usize, 0_usize);
        let mut ans = -1;
        let mut acc = 0;

        if target < 0 {
            return -1;
        }

        while right < n {
            acc += nums[right];
            while acc > target {
                acc -= nums[left];
                left += 1;
            }
            if acc == target {
                ans = max(ans, (right - left + 1) as i32);
            }
            right += 1;
        }
        if ans == -1 {
            return ans;
        }
        n as i32 - ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1658() {
        let nums = vec![1, 1, 4, 2, 3];
        assert_eq!(Solution::min_operations(nums, 5), 2);
        let nums = vec![5, 6, 7, 8, 9];
        assert_eq!(Solution::min_operations(nums, 4), -1);
    }
}
