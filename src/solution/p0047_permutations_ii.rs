pub struct Solution {}

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::<Vec<i32>>::new();
        let mut temp = Vec::new();
        let mut marks = Vec::<bool>::new();
        marks.resize(nums.len(), false);
        temp.resize(nums.len(), 0);
        nums.sort();

        fn dfs(
            nums: &Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
            idx: usize,
            temp: &mut Vec<i32>,
            marks: &mut Vec<bool>,
        ) {
            println!("{:?}", temp);
            if idx == nums.len() {
                ans.push(temp.clone());
                return;
            }
            for i in 0..marks.len() {
                if marks[i] == false {
                    if i > 0 {
                        if nums[i] == nums[i - 1] && marks[i - 1] == false {
                            continue;
                        }
                    }
                    marks[i] = true;
                    temp[idx] = nums[i];
                    dfs(nums, ans, idx + 1, temp, marks);
                    marks[i] = false;
                }
            }
        }
        dfs(&nums, &mut ans, 0, &mut temp, &mut marks);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_47() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
    }
}
