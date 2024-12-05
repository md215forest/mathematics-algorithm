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
        vec![1],
        vec![0, 5, 10],
        vec![5, 10],
        vec![1, 11],
        vec![7, 11],
        vec![1, 2, 13],
        vec![12],
        vec![3, 4, 14],
        vec![10],
        vec![11],
        vec![1, 2, 8],
        vec![3, 4, 9],
        vec![6, 13, 14],
        vec![5, 12],
        vec![7, 12],
    ];

    let mut visited = HashSet::new();
    dfs(&graph, 0, &mut visited);

    visited.len() == graph.len()
}
