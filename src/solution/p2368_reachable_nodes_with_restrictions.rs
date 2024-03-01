pub struct Solution {}

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let mut graph: Vec<Vec<i32>> = vec![Vec::new(); n as usize];
        let mut mark = vec![0; n as usize];
        for i in restricted {
            mark[i as usize] = 1;
        }
        for edge in edges.iter() {
            let (u, v) = (edge[0], edge[1]);
            graph[u as usize].push(v);
            graph[v as usize].push(u);
        }
        let mut ans = 0;
        fn dfs(cur: i32, father: i32, graph: &Vec<Vec<i32>>, mark: &Vec<i32>, ans: &mut i32) {
            *ans += 1;
            for &x in graph[cur as usize].iter() {
                if mark[x as usize] == 0 && x != father {
                    dfs(x, cur, graph, mark, ans);
                }
            }
        }
        dfs(0, -1, &graph, &mark, &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2368() {
        assert_eq!(
            Solution::reachable_nodes(
                7,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![3, 1],
                    vec![4, 0],
                    vec![0, 5],
                    vec![5, 6]
                ],
                vec![4, 5]
            ),
            4
        );
    }
}
