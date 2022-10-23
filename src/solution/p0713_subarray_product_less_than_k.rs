pub struct Solution {}

impl Solution {
    pub fn num_subarray_product_less_than_k(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        let mut product = 1;
        let mut i = 0;
        for (j, num) in nums.iter().enumerate() {
            product *= num;
            while i <= j && product >= k {
                product /= nums[i];
                i += 1;
            }
            ans += j - i + 1;
        }

        ans as i32
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert_eq!(
        //     Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
        //     8
        // );
        //
        // let a = Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0);
        // println!("{}", a);
    }
}
