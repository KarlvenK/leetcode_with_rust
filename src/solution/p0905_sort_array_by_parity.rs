pub struct Solution {}
impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let (mut i, mut j) = (0, nums.len() - 1);
        while i < j {
            while i < j && nums[i] & 1 == 0 {
                i += 1;
            }
            while i < j && nums[j] & 1 == 1 {
                j -= 1;
            }
            let temp = nums[i];
            nums[i] = nums[j];
            nums[j] = temp;
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_905() {
        let a = vec![3, 1, 2, 4];
        assert_eq!(Solution::sort_array_by_parity(a), vec![4, 2, 1, 3]);
    }
}
