pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0_usize;
        while i < nums.len() {
            if nums[i] == val {
                if i < nums.len() - 1 {
                    nums[i] = nums.pop().unwrap();
                } else {
                    nums.pop().unwrap();
                    continue;
                }
            }
            if nums[i] != val {
                i += 1;
            }
        }
        println!("{}", nums.len());
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27() {
        assert_eq!(
            Solution::remove_element(&mut vec![1, 2, 2, 2, 2, 2, 2], 2),
            1
        );
        assert_eq!(
            Solution::remove_element(&mut vec![2, 2, 2, 2, 2, 2, 2], 2),
            0
        );
        assert_eq!(Solution::remove_element(&mut vec![1], 1), 0);
        let mut vec1 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut vec1, 2), 5);
        assert_eq!(vec1[0..5], [0, 1, 4, 0, 3]);
        assert_eq!(Solution::remove_element(&mut vec![], 2), 0);
    }
}
