pub struct Solution {}

impl Solution {
    fn dfs(graph: &mut Vec<Vec<i32>>, color: &mut Vec<i32>, u: i32, p: i32, ans: &mut bool) {
        if ans == &false {
            return;
        }
        if color[u as usize] == -p {
            *ans = false;
        }
        if color[u as usize] == p {
            return;
        }
        color[u as usize] = p;
        for i in 0..graph[u as usize].len() {
            Solution::dfs(graph, color, graph[u as usize][i], -p, ans);
        }
    }

    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut ans: bool = true;

        let mut graph: Vec<Vec<i32>> = vec![];
        graph.resize_with(n as usize + 1, || {
            let ret: Vec<i32> = Vec::new();
            ret
        });

        for edge in dislikes {
            let (u, v) = (edge[0], edge[1]);
            graph[u as usize].push(v);
            graph[v as usize].push(u);
        }

        let mut color = vec![0; n as usize + 1];

        for i in 1..=n {
            if color[i as usize] == 0 {
                Solution::dfs(&mut graph, &mut color, i, 1, &mut ans);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_886() {
        assert_eq!(true, Solution::possible_bipartition(4, vec![vec![1,2], vec![1, 3], vec![2, 4]]));
        assert_eq!(false, Solution::possible_bipartition(3, vec![vec![1,2], vec![1, 3], vec![2, 3]]));
    }
}
