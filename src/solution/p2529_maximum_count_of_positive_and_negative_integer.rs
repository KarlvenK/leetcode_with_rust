use std::cmp::max;

struct Solution {}
impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let pos_ge_zero = Self::lower_bound(&nums, 0);
        let pos_ge_one = Self::lower_bound(&nums, 1);

        match pos_ge_zero {
            None => return nums.len() as i32,
            Some(p0) => match pos_ge_one {
                None => return p0 as i32,
                Some(p1) => return max(p0, nums.len() - p1) as i32,
            },
        }
    }
    fn lower_bound(nums: &Vec<i32>, target: i32) -> Option<usize> {
        let (mut left, mut right) = (0_i32, nums.len() as i32 - 1);
        let mut ans = None;

        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] >= target {
                ans = Some(mid as usize);
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test2529() {
        assert_eq!(4, Solution::maximum_count(vec![5, 20, 66, 1314]));
        assert_eq!(3, Solution::maximum_count(vec![-2, -1, 0, 0, 1, 2, 3]));
        assert_eq!(0, Solution::maximum_count(vec![0, 0, 0]));
    }
}
