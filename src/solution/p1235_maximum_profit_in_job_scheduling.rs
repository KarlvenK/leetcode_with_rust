use std::cmp::max;

pub struct Solution {}
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = (0..start_time.len())
            .map(|i| vec![start_time[i], end_time[i], profit[i]])
            .collect::<Vec<_>>();
        jobs.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut dp = vec![0; start_time.len() + 1];

        for i in 1..=start_time.len() {
            let k = Solution::binary_search(&jobs, i - 1);
            dp[i] = max(dp[i - 1], jobs[i - 1][2] + dp[k]);
        }

        dp[start_time.len()]
    }

    fn binary_search(jobs: &Vec<Vec<i32>>, target: usize) -> usize {
        let (mut left, mut right) = (0usize, target);
        while left + 1 < right {
            let mid = (left + right) >> 1;
            if jobs[mid][1] <= jobs[target][0] {
                left = mid;
            } else {
                right = mid;
            }
        }
        if jobs[left][1] <= jobs[target][0] {
            return left + 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1235() {
        assert_eq!(
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            ),
            150
        );
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120
        );
    }
}
