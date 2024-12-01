use std::collections::VecDeque;

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> usize {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        println!("Visiting node {}", node);

        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    visited.iter().filter(|&&x| x).count()
}

pub fn run() -> bool {
    // グラフの定義（隣接リスト形式）
    let graph = vec![
        vec![1, 2],    // ノード0に隣接するノード
        vec![0, 3, 4], // ノード1に隣接するノード
        vec![0, 5],    // ノード2に隣接するノード
        vec![1],       // ノード3に隣接するノード
        vec![1, 5],    // ノード4に隣接するノード
        vec![2, 4],    // ノード5に隣接するノード
    ];

    let visited_count = bfs(&graph, 0);
    visited_count == graph.len()
}
