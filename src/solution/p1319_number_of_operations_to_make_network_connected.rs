pub struct Solution {}

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        if n - 1 > connections.len() as i32 {
            return -1;
        }
        let mut graph = Vec::new();
        for _i in 0..n as usize {
            graph.push(Vec::new());
        }
        let mut vis = Vec::new();
        for _i in 0..n as usize {
            vis.push(false);
        }
        for edge in connections {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        fn dfs(graph: &Vec<Vec<usize>>, vis: &mut Vec<bool>, node: usize) {
            if vis[node] {
                return;
            }
            vis[node] = true;
            for v in &graph[node] {
                dfs(graph, vis, *v);
            }
        }
        let mut cnt = 0;
        for i in 0..n as usize {
            if !vis[i] {
                dfs(&graph, &mut vis, i);
                cnt += 1;
            }
        }
        cnt - 1
    }
}
