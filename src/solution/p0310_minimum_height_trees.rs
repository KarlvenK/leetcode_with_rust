use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let n = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        let mut ind = vec![0; n];
        for edge in edges.iter() {
            graph[edge[0] as usize].push(edge[1] as usize);
            graph[edge[1] as usize].push(edge[0] as usize);
            ind[edge[0] as usize] += 1;
            ind[edge[1] as usize] += 1;
        }
        let mut que = VecDeque::new();
        for (i, &n) in ind.iter().enumerate() {
            if n == 1 {
                que.push_back(i)
            }
        }
        let mut cnt = n as i32;
        while cnt > 2 {
            cnt -= que.len() as i32;
            for _ in 0..que.len() {
                let u = que.pop_front().unwrap();
                for &v in &graph[u] {
                    ind[v] -= 1;
                    if ind[v] == 1 {
                        que.push_back(v)
                    }
                }
            }
        }
        let mut ans = vec![];
        while que.len() != 0 {
            ans.push(que.pop_front().unwrap() as i32);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_310() {
        assert_eq!(
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
            vec![1]
        );
        assert_eq!(
            Solution::find_min_height_trees(
                6,
                vec![vec![0, 3], vec![1, 3], vec![2, 3], vec![4, 3], vec![5, 4]]
            ),
            vec![3, 4]
        );
        assert_eq!(Solution::find_min_height_trees(1, vec![]), vec![0]);
    }
}
