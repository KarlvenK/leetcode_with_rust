struct Solution {}
impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        for i in 0..n {
            let sum = grid[i].iter().fold(0, |acc, x| acc + x);
            if sum == n as i32 - 1 {
                return i as i32;
            }
        }
        0
    }
}
