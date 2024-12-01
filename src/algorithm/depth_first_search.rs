use std::collections::HashSet;

fn dfs(graph: &Vec<Vec<usize>>, start: usize, visited: &mut HashSet<usize>) {
    if visited.contains(&start) {
        return;
    }

    println!("Visiting node {}", start);
    visited.insert(start);

    for &neighbor in &graph[start] {
        dfs(graph, neighbor, visited);
    }
}

pub fn run() -> bool {
    let graph = vec![
        vec![1, 2],    // ノード0に隣接するノード
        vec![0, 3, 4], // ノード1に隣接するノード
        vec![0, 5, 6], // ノード2に隣接するノード
        vec![1],       // ノード3に隣接するノード
        vec![1, 5],    // ノード4に隣接するノード
        vec![2, 4],    // ノード5に隣接するノード
        vec![2],       // ノード6に隣接するノード
    ];

    let mut visited = HashSet::new();
    dfs(&graph, 0, &mut visited);

    visited.len() == graph.len()
}
