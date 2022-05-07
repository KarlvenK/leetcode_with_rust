pub struct Solution {}

fn abs(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}
impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let n = nums.len();
        for i in 0..n {
            let x = (abs(nums[i]) - 1) as usize;
            if nums[x] < 0 {
                ans.push(abs(nums[i]));
            }
            nums[x] *= -1;
        }
        ans
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_442() {
        assert_eq!(
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
        assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1]);
        assert_eq!(Solution::find_duplicates(vec![1]), vec![]);
    }
}
