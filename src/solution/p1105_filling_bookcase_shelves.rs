use std::cmp::{max, min};

pub struct Solution {}

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![1000000; n + 1];
        dp[0] = 0;
        /*
        dp[i] 表示前i本书（编号从1开始）的最小高度
        dp[i] = min(dp[j], max(books[k])
        */

        for i in 1..=n {
            let (mut sum, mut m) = (0, 0);
            for j in (0..i).rev() {
                sum += books[j][0];
                if sum > shelf_width {
                    break;
                }
                m = max(m, books[j][1]);
                dp[i] = min(dp[i], dp[j] + m);
            }
        }
        return dp[n];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1105() {
        let books: Vec<Vec<i32>> = [[1, 1], [2, 3], [2, 3], [1, 1], [1, 1], [1, 1], [1, 2]]
            .iter()
            .map(|x| x.to_vec())
            .collect();
        assert_eq!(Solution::min_height_shelves(books, 4), 6)
    }
}
