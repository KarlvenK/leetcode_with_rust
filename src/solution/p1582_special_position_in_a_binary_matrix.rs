pub struct Solution {}
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let matrix = mat;
        let row_num = matrix.len();
        let column_num = matrix[0].len();
        let mut row_cnt = vec![0; row_num];
        let mut column_cnt = vec![0; column_num];

        for i in 0..row_num {
            for j in 0..column_num {
                if matrix[i][j] == 1 {
                    row_cnt[i] += 1;
                    column_cnt[j] += 1;
                }
            }
        }
        let mut ans = 0;
        for i in 0..row_num {
            for j in 0..column_num {
                if matrix[i][j] == 1 {
                    if row_cnt[i] == 1 && column_cnt[j] == 1 {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1582() {
        let mat = vec![
            vec![0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 1],
            vec![0, 0, 0, 0, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::num_special(mat), 1);
    }
}
