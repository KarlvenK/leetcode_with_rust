struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 1..=target {
            for &num in nums.iter() {
                if i >= num {
                    dp[i as usize] += dp[(i - num) as usize];
                }
            }
        }
        dp[target as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_377() {
        assert_eq!(7, Solution::combination_sum4(vec![1, 2, 3], 4))
    }
}
