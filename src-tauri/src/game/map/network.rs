use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use delaunator::{Point, triangulate};

/// 使用 Delaunay 三角剖分生成候选边集
pub(super) fn build_delaunay_edges(positions: &[(f64, f64)]) -> Vec<(usize, usize)> {
    let n = positions.len();
    if n < 3 {
        let mut edges = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                edges.push((i, j));
            }
        }
        return edges;
    }

    let points: Vec<Point> = positions.iter()
        .map(|&(x, y)| Point { x, y })
        .collect();

    let result = triangulate(&points);

    let mut edge_set = std::collections::HashSet::new();
    for chunk in result.triangles.chunks(3) {
        if chunk.len() < 3 {
            continue;
        }
        let a = chunk[0];
        let b = chunk[1];
        let c = chunk[2];
        edge_set.insert((a.min(b), a.max(b)));
        edge_set.insert((b.min(c), b.max(c)));
        edge_set.insert((c.min(a), c.max(a)));
    }

    edge_set.into_iter().collect()
}

/// 过滤长度超过中位数 3 倍的边
pub(super) fn filter_long_edges(
    edges: &[(usize, usize)],
    positions: &[(f64, f64)],
) -> Vec<(usize, usize)> {
    let m = edges.len();
    if m == 0 {
        return Vec::new();
    }

    let mut edge_lengths: Vec<(f64, (usize, usize))> = edges.iter()
        .map(|&(i, j)| {
            let dx = positions[i].0 - positions[j].0;
            let dy = positions[i].1 - positions[j].1;
            let d = (dx * dx + dy * dy).sqrt();
            (d, (i, j))
        })
        .collect();

    edge_lengths.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let median = edge_lengths[m / 2].0;
    let threshold = median * 3.0;

    edge_lengths.into_iter()
        .filter(|&(d, _)| d <= threshold)
        .map(|(_, e)| e)
        .collect()
}

// ── 并查集 ──────────────────────────────────────

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return false;
        }
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else {
            self.parent[ry] = rx;
            self.rank[rx] += 1;
        }
        true
    }
}

/// 确保边集使所有点连通：对每个孤立分量补最短跨分量边
pub(super) fn ensure_connectivity(
    edges: &[(usize, usize)],
    positions: &[(f64, f64)],
) -> Vec<(usize, usize)> {
    let n = positions.len();
    if n <= 1 {
        return Vec::new();
    }

    let mut result: Vec<(usize, usize)> = edges.to_vec();
    let mut uf = UnionFind::new(n);

    for &(i, j) in &result {
        uf.union(i, j);
    }

    loop {
        let mut comps: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n {
            let root = uf.find(i);
            comps.entry(root).or_default().push(i);
        }
        if comps.len() <= 1 {
            break;
        }

        let comp_list: Vec<Vec<usize>> = comps.into_values().collect();

        let mut best_pair = (0, 0);
        let mut best_dist = f64::MAX;

        for ci in 0..comp_list.len() {
            for cj in (ci + 1)..comp_list.len() {
                for &a in &comp_list[ci] {
                    for &b in &comp_list[cj] {
                        let dx = positions[a].0 - positions[b].0;
                        let dy = positions[a].1 - positions[b].1;
                        let d = dx * dx + dy * dy;
                        if d < best_dist {
                            best_dist = d;
                            best_pair = (a, b);
                        }
                    }
                }
            }
        }

        result.push(best_pair);
        uf.union(best_pair.0, best_pair.1);
    }

    result
}

// ── 冗余边过滤 ──────────────────────────────────

/// Dijkstra 状态
#[derive(Clone, PartialEq)]
struct DijkState {
    cost: f64,
    node: usize,
}

impl Eq for DijkState {}
impl Ord for DijkState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for DijkState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Dijkstra 最短路径距离，跳过指定边
fn dijkstra_skip_edge(
    adj: &[Vec<(usize, f64)>],
    start: usize,
    goal: usize,
    skip: (usize, usize),
) -> Option<f64> {
    let n = adj.len();
    let mut dist = vec![f64::MAX; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0.0;
    heap.push(DijkState { cost: 0.0, node: start });

    while let Some(state) = heap.pop() {
        if state.node == goal {
            return Some(state.cost);
        }
        if state.cost > dist[state.node] {
            continue;
        }
        for &(next, weight) in &adj[state.node] {
            // 跳过被测试的边（双向）
            if (state.node == skip.0 && next == skip.1)
                || (state.node == skip.1 && next == skip.0)
            {
                continue;
            }
            let new_cost = state.cost + weight;
            if new_cost < dist[next] {
                dist[next] = new_cost;
                heap.push(DijkState { cost: new_cost, node: next });
            }
        }
    }

    None
}

/// 过滤冗余边：如果从 i 到 j 存在替代路径（不经过该边），
/// 且替代路径长度 ≤ 直接距离 × 1.3，则删除该边
pub(super) fn filter_redundant_edges(
    edges: &[(usize, usize)],
    positions: &[(f64, f64)],
) -> Vec<(usize, usize)> {
    let n = positions.len();
    let m = edges.len();
    if m == 0 {
        return Vec::new();
    }

    // 构建带权邻接表（权重 = 欧几里得距离）
    let mut adj: Vec<Vec<(usize, f64)>> = vec![vec![]; n];
    for &(i, j) in edges {
        let dx = positions[i].0 - positions[j].0;
        let dy = positions[i].1 - positions[j].1;
        let d = (dx * dx + dy * dy).sqrt();
        adj[i].push((j, d));
        adj[j].push((i, d));
    }

    // 标记冗余边
    let mut redundant = vec![false; m];
    for (idx, &(i, j)) in edges.iter().enumerate() {
        let dx = positions[i].0 - positions[j].0;
        let dy = positions[i].1 - positions[j].1;
        let direct = (dx * dx + dy * dy).sqrt();

        if let Some(alt) = dijkstra_skip_edge(&adj, i, j, (i, j)) {
            // 替代路径不超过直接距离的 1.3 倍 → 冗余
            if alt <= direct * 1.3 {
                redundant[idx] = true;
            }
        }
    }

    // 保留非冗余边
    edges.iter()
        .enumerate()
        .filter(|(idx, _)| !redundant[*idx])
        .map(|(_, &e)| e)
        .collect()
}
