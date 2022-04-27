pub struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let (m, n) = (heights.len(), heights[0].len());
        let dir = vec![vec![1, 0], vec![0, 1], vec![-1, 0], vec![0, -1]];
        let mut pacific = vec![vec![false; n]; m];
        let mut atlantic = vec![vec![false; n]; m];

        struct Env<'a> {
            m: usize,
            n: usize,
            dir: &'a Vec<Vec<i32>>,
            height: &'a Vec<Vec<i32>>,
        }
        let env = Env {
            m,
            n,
            dir: &dir,
            height: &heights,
        };

        fn dfs(x: i32, y: i32, ocean: &mut Vec<Vec<bool>>, env: &Env) {
            if ocean[x as usize][y as usize] {
                return;
            }
            ocean[x as usize][y as usize] = true;
            for delta in env.dir {
                let (nx, ny) = (x + delta[0], y + delta[1]);
                if nx < 0 || ny < 0 || nx >= (env.m as i32) || ny >= (env.n as i32) {
                    continue;
                }
                if env.height[nx as usize][ny as usize] >= env.height[x as usize][y as usize] {
                    dfs(nx, ny, ocean, env);
                }
            }
        }

        for i in 0..m {
            dfs(i as i32, 0, &mut pacific, &env);
        }
        for i in 0..n {
            dfs(0, i as i32, &mut pacific, &env);
        }
        for i in 0..m {
            dfs(i as i32, n as i32 - 1, &mut atlantic, &env);
        }
        for i in 0..n {
            dfs(m as i32 - 1, i as i32, &mut atlantic, &env);
        }

        for i in 0..m {
            for j in 0..n {
                if pacific[i][j] && atlantic[i][j] {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_417() {
        let mut a = Solution::pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ]);
        a.sort();
        let mut b = vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ];
        b.sort();
        assert_eq!(a, b);

        let mut a = Solution::pacific_atlantic(vec![vec![2, 1], vec![1, 2]]);
        a.sort();
        let mut b = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]];
        b.sort();
        assert_eq!(a, b);
    }
}
