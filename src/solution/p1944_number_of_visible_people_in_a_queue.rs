use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut ans = vec![0; n];
        let mut stack = VecDeque::<usize>::new();
        for (i, &v) in heights.iter().enumerate().rev() {
            let mut cnt = 0;
            while let Some(idx) = stack.back() {
                cnt += 1;
                if v < heights[*idx] {
                    break;
                }
                stack.pop_back();
            }
            stack.push_back(i);
            ans[i] = cnt;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1944() {
        assert_eq!(
            vec![3, 1, 2, 1, 1, 0],
            Solution::can_see_persons_count(vec![10, 6, 8, 5, 11, 9])
        );
        assert_eq!(
            vec![4, 1, 1, 1, 0],
            Solution::can_see_persons_count(vec![5, 1, 2, 3, 10])
        )
    }
}
