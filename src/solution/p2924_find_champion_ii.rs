struct Solution;

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut in_degree = vec![0; n as usize];
        let mut ans = 0;
        edges.iter().for_each(|e| {
            in_degree[e[1] as usize] += 1;
        });
        let cnt = in_degree.iter().enumerate().fold(0, |acc, (idx, x)| {
            if *x != 0 {
                acc
            } else {
                ans = idx;
                acc + 1
            }
        });
        if cnt == 1 {
            ans as i32
        } else {
            -1
        }
    }
}
