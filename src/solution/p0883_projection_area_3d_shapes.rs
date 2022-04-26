use std::cmp::max;
pub struct Solution {}
impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ans = 0;
        let (mut x_max, mut y_max) = (0, 0);

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] > 0 {
                    ans += 1;
                }
            }
        }

        for i in 0..n {
            let mut x_height = 0;
            for j in 0..n {
                x_height = max(grid[i][j], x_height);
            }
            x_max += x_height;
        }

        for j in 0..n {
            let mut y_height = 0;
            for i in 0..n {
                y_height = max(y_height, grid[i][j]);
            }
            y_max += y_height;
        }

        ans += x_max + y_max;
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_833() {
        assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
        assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
    }
}
