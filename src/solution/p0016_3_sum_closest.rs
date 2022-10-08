pub struct Solution {}

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        fn abs(x: i32) -> i32 {
            if x < 0 {
                return -x;
            }
            return x;
        }

        nums.sort_unstable();
        let n = nums.len();
        let mut ans = 10000009;
        for i in 0..n {
            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == target {
                    return target;
                }
                if abs(ans - target) > abs(sum - target) {
                    ans = sum;
                }
                if sum > target {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }

        ans
    }
}
