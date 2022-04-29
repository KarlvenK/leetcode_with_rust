pub struct Solution {}
impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        if nums.last().unwrap() - nums.first().unwrap() <= (2 * k) {
            return 0;
        }
        nums.last().unwrap() - nums.first().unwrap() - 2 * k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_908() {
        assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
        assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
        assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
    }
}
