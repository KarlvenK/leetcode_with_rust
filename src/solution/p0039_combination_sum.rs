pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = candidates;
        let mut res: Vec<Vec<i32>> = Vec::new();
        nums.sort_unstable_by(|a, b| b.cmp(a));
        let mut v = Vec::new();

        Solution::dfs(&nums, target, &mut v, &mut res, 0);
        println!("{:?}", res);
        res
    }

    fn dfs(nums: &Vec<i32>, target: i32, curr: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, idx: usize) {
        if idx >= nums.len() {
            return;
        }
        if target == 0 {
            res.push(curr.to_vec());
            return;
        }
        Solution::dfs(nums, target, curr, res, idx + 1);
        if target - nums[idx] >= 0 {
            curr.push(nums[idx]);
            Solution::dfs(nums, target - nums[idx], curr, res, idx);
            curr.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_39() {
        assert_eq!(
            Solution::combination_sum(vec![1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![3, 2, 2], vec![7]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![3, 3, 2], vec![5, 3]]
        );
    }
}
