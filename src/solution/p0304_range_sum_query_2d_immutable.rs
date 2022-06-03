pub struct Solution {}

struct NumMatrix {
    acc: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut ret = NumMatrix { acc: Vec::new() };
        ret.acc.resize(m + 1, Vec::<i32>::new());
        for i in 0..=m {
            ret.acc[i].resize(n + 1, 0);
        }
        for i in 0..m {
            for j in 0..n {
                ret.acc[i + 1][j + 1] =
                    ret.acc[i + 1][j] + ret.acc[i][j + 1] - ret.acc[i][j] + matrix[i][j];
            }
        }
        ret
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, row2, col1, col2) = (row1 as usize, row2 as usize, col1 as usize, col2 as usize);
        self.acc[row2 + 1][col2 + 1] + self.acc[row1][col1]
            - self.acc[row2 + 1][col1]
            - self.acc[row1][col2 + 1]
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

mod tests {
    use super::*;

    #[test]
    fn test_304() {
        let matrix = vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ];
        let obj = NumMatrix::new(matrix);
        assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
    }
}
