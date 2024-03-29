use std::cmp::min;

struct Solution {}

impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left_min = vec![9999; n];
        let mut right_min = vec![9999; n];
        let mut ans = -1;
        left_min[0] = nums[0];
        right_min[n - 1] = nums[n - 1];
        for i in 1..n {
            left_min[i] = min(left_min[i - 1], nums[i - 1]);
        }
        for i in (0..n - 1).rev() {
            right_min[i] = min(right_min[i + 1], nums[i + 1]);
        }
        for i in 1..n - 1 {
            if right_min[i] < nums[i] && left_min[i] < nums[i] {
                if ans == -1 {
                    ans = right_min[i] + nums[i] + left_min[i];
                } else {
                    ans = min(ans, right_min[i] + nums[i] + left_min[i]);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2908() {
        assert_eq!(9, Solution::minimum_sum(vec![8, 6, 1, 5, 3]))
    }
}
