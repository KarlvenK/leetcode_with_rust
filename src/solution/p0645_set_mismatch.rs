pub struct Solution {}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let abs = |n: i32| {
            if n < 0 {
                -n
            } else {
                n
            }
        };
        let n = nums.len() as i32;
        let mut ans = vec![0; 2];
        for i in 0..(n as usize) {
            if nums[abs(nums[i]) as usize - 1] < 0 {
                ans[0] = abs(nums[i]);
            } else {
                let idx = abs(nums[i]) as usize - 1;
                nums[idx] *= -1;
            }
        }

        for i in 0..(n as usize) {
            if nums[i] > 0 {
                ans[1] = i as i32 + 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_645() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
        assert_eq!(
            Solution::find_error_nums(vec![1, 2, 3, 4, 4, 5]),
            vec![4, 6]
        );
    }
}
