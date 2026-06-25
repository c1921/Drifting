use std::collections::BinaryHeap;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use serde::Serialize;

// ── 地图常量 ──────────────────────────────────────
pub const MAP_WIDTH: u32 = 512;
pub const MAP_HEIGHT: u32 = 512;
pub const MAP_SCALE: f64 = 0.008; // 噪声缩放（越小特征越大）
pub const OCTAVES: usize = 4;     // FBM 层数（越少细节越平滑）
pub const PERSISTENCE: f64 = 0.5;
pub const LACUNARITY: f64 = 2.0;

/// A* 坡度惩罚系数：越大则路径越倾向于绕开陡坡（即使路径变长）
const SLOPE_PENALTY: f64 = 300.0;

// ── 前端期望的坐标系范围 ──────────────────────────
// 前端地图在 [-400, 400] x [-300, 300] 范围内
const WORLD_MIN: f64 = -800.0;
const WORLD_MAX: f64 = 800.0;

// ── 数据结构 ──────────────────────────────────────

/// 地点类型（匹配前端 LocationData.type）
/// Large = 城市, Medium = 小镇, Small = 村庄
#[derive(Debug, Clone, Serialize)]
pub enum LocationType {
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "small")]
    Small,
}

/// 单个地点
#[derive(Debug, Clone, Serialize)]
pub struct LocationData {
    pub id: String,
    pub name: String,
    pub x: f64,
    pub y: f64,
    #[serde(rename = "type")]
    pub loc_type: LocationType,
}

/// 道路折线（前端 flat array: [x1,y1,x2,y2,...]）
#[derive(Debug, Clone, Serialize)]
pub struct RoadData {
    pub points: Vec<f64>,
}

/// 返回给前端的地图数据
#[derive(Debug, Clone, Serialize)]
pub struct MapData {
    /// 高度图一维数组，行主序，每个值 0.0~1.0
    pub heightmap: Vec<f32>,
    pub width: u32,
    pub height: u32,
    pub locations: Vec<LocationData>,
    pub roads: Vec<RoadData>,
}

// ── 名称池 ──────────────────────────────────────

// ── 名称池（按地点规模分级）────────────────────

const LARGE_NAMES: &[&str] = &[
    "Aetheris", "Bastion", "Celestia", "Dragonport", "Everbright",
    "Goldspire", "Highreach", "Ironhold", "King's Landing", "Luminara",
    "Meridian", "Northgate", "Obsidian", "Palisade", "Queensford",
];

const MEDIUM_NAMES: &[&str] = &[
    "Ashford", "Briarwood", "Copperford", "Dawnhold", "Eastwatch",
    "Fairhaven", "Glenwood", "Hollowshire", "Ivorygate", "Jadeport",
    "Kingsford", "Lunaris", "Maplecrest", "Northbrook", "Oakhaven",
    "Pinehaven", "Riverton", "Silverbrook", "Starlight", "Stonebridge",
    "Thornwall", "Willowdale", "Westmarch", "Wyncrest", "Millbrook",
];

const SMALL_NAMES: &[&str] = &[
    "Applewood", "Birch Hollow", "Cobblestone", "Dusty Creek", "Elmstead",
    "Fernbank", "Greenvale", "Hawthorn", "Ivy Cottage", "Juniper",
    "Larkspur", "Mossy Rock", "New Dawn", "Oakleaf", "Primrose",
    "Quiet Pond", "Red Deer", "Sunny Vale", "Timber Mill", "Underhill",
    "Violet Field", "Wild Rose", "Yarrow", "Zephyr Cove", "Amber Glade",
];

// ── 主要生成函数 ──────────────────────────────────

/// 生成完整地图数据
pub fn generate_map(seed: u32) -> MapData {
    let seed = if seed == 0 {
        rand::thread_rng().gen()
    } else {
        seed
    };

    let heightmap = generate_heightmap(seed);
    let locations = generate_locations(&heightmap, seed);
    let roads = generate_roads(&locations, &heightmap);

    MapData {
        heightmap: heightmap.into_iter().map(|v| v as f32).collect(),
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
        locations,
        roads,
    }
}

// ── 高度图生成 ──────────────────────────────────

/// 使用 FBM Perlin 噪声生成高度网格，返回 0.0~1.0
fn generate_heightmap(seed: u32) -> Vec<f64> {
    let perlin = Perlin::new(seed as u32);

    let mut heights = Vec::with_capacity((MAP_WIDTH * MAP_HEIGHT) as usize);

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let nx = x as f64 * MAP_SCALE;
            let ny = y as f64 * MAP_SCALE;

            // FBM 手动实现
            let mut value = 0.0;
            let mut amplitude = 1.0;
            let mut frequency = 1.0;
            let mut max_amplitude = 0.0;

            for _ in 0..OCTAVES {
                value += perlin.get([nx * frequency, ny * frequency]) * amplitude;
                max_amplitude += amplitude;
                amplitude *= PERSISTENCE;
                frequency *= LACUNARITY;
            }

            // 归一化到 0~1，再应用乘方映射使更多地形落入低海拔
            let normalized = (value / max_amplitude + 1.0) * 0.5;
            let clamped = normalized.clamp(0.0, 1.0).powf(3.0);
            heights.push(clamped);
        }
    }

    heights
}

/// 获取高度图上 (x, y) 处的高度值
fn get_height(heights: &[f64], x: u32, y: u32) -> f64 {
    let idx = (y * MAP_WIDTH + x) as usize;
    heights[idx.min(heights.len() - 1)]
}

/// 将像素坐标转换为世界坐标（地图中心为 (0,0)）
fn pixel_to_world(px: u32, py: u32) -> (f64, f64) {
    let wx = (px as f64 / MAP_WIDTH as f64) * (WORLD_MAX - WORLD_MIN) + WORLD_MIN;
    let wy = (py as f64 / MAP_HEIGHT as f64) * (WORLD_MAX - WORLD_MIN) + WORLD_MIN;
    (wx, wy)
}

/// 将世界坐标转换为像素坐标（四舍五入取整）
fn world_to_pixel(wx: f64, wy: f64) -> (u32, u32) {
    let px = ((wx - WORLD_MIN) / (WORLD_MAX - WORLD_MIN) * MAP_WIDTH as f64).round() as u32;
    let py = ((wy - WORLD_MIN) / (WORLD_MAX - WORLD_MIN) * MAP_HEIGHT as f64).round() as u32;
    (px.clamp(0, MAP_WIDTH - 1), py.clamp(0, MAP_HEIGHT - 1))
}

// ── 地点生成 ──────────────────────────────────

/// 在地图上按高度分层生成大/中/小三类地点
/// Large（城市）→ 平原 h<0.20
/// Medium（小镇）→ 低海拔 h<0.35
/// Small（村庄）→ 可到中海拔 h<0.50
fn generate_locations(heights: &[f64], seed: u32) -> Vec<LocationData> {
    let mut locations = Vec::new();
    let mut loc_rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(
        [seed as u8; 32]
    );

    // 目标：50~60 个地点
    let max_attempts = 1000;
    let target_count = 50 + (seed as usize % 11); // 50~60

    // 各类型目标占比：Large ~15%, Medium ~40%, Small ~45%
    let mut attempts = 0;
    while locations.len() < target_count && attempts < max_attempts {
        attempts += 1;

        let px = loc_rng.gen_range(40..MAP_WIDTH - 40);
        let py = loc_rng.gen_range(40..MAP_HEIGHT - 40);
        let h = get_height(heights, px, py);

        // 仅限低海拔平原（<0.10）
        if h >= 0.10 {
            continue;
        }

        // 检查间距（至少 40 世界单位）
        let (wx, wy) = pixel_to_world(px, py);
        let too_close = locations.iter().any(|l: &LocationData| {
            let dx = l.x - wx;
            let dy = l.y - wy;
            (dx * dx + dy * dy) < 1600.0 // 40^2
        });
        if too_close {
            continue;
        }

        // 按高度决定地点规模
        let loc_type = if h < 0.20 {
            // 平原：城市或小镇
            if loc_rng.gen_bool(0.30) {
                LocationType::Large
            } else {
                LocationType::Medium
            }
        } else if h < 0.35 {
            // 低海拔：小镇或村庄
            if loc_rng.gen_bool(0.40) {
                LocationType::Medium
            } else {
                LocationType::Small
            }
        } else {
            // 中海拔：仅限村庄
            LocationType::Small
        };

        let name = pick_name(&loc_type, &mut loc_rng);
        let id = format!("loc_{}", locations.len() + 1);

        locations.push(LocationData {
            id,
            name,
            x: (wx * 10.0).round() / 10.0,
            y: (wy * 10.0).round() / 10.0,
            loc_type,
        });
    }

    // ── 兜底：确保至少有 4 个城市 ─────────────────
    let large_count = locations.iter().filter(|l| matches!(l.loc_type, LocationType::Large)).count();
    if large_count < 4 {
        let needed = 4 - large_count;
        // 从 Medium 中随机选 needed 个升级为 Large
        let medium_indices: Vec<usize> = locations.iter().enumerate()
            .filter(|(_, l)| matches!(l.loc_type, LocationType::Medium))
            .map(|(i, _)| i)
            .collect();
        let mut rng = rand::thread_rng();
        let upgrade_count = needed.min(medium_indices.len());
        let chosen = if upgrade_count < medium_indices.len() {
            let mut indices = medium_indices;
            // 打乱后取前 upgrade_count 个
            for i in (1..indices.len()).rev() {
                let j = rng.gen_range(0..=i);
                indices.swap(i, j);
            }
            indices[..upgrade_count].to_vec()
        } else {
            medium_indices
        };
        for &idx in &chosen {
            locations[idx].loc_type = LocationType::Large;
            locations[idx].name = pick_name(&LocationType::Large, &mut loc_rng);
        }
    }

    locations
}

/// 从对应规模的名称池中选取一个名字
fn pick_name(loc_type: &LocationType, rng: &mut impl Rng) -> String {
    let pool = match loc_type {
        LocationType::Large => LARGE_NAMES,
        LocationType::Medium => MEDIUM_NAMES,
        LocationType::Small => SMALL_NAMES,
    };
    let idx = rng.gen_range(0..pool.len());
    pool[idx].to_string()
}

// ── 分层道路网络生成 ──────────────────────────────

/// 并查集（Union-Find），用于最小生成树
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

    /// 合并两个集合，返回 true 表示合并成功（原本不连通）
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

/// 尝试添加一条道路，若两端未超过连接上限则成功
fn try_add_road(
    roads: &mut Vec<RoadData>,
    conn: &mut [u32],
    locations: &[LocationData],
    heights: &[f64],
    i: usize,
    j: usize,
) -> bool {
    let max_i = match locations[i].loc_type {
        LocationType::Large => 5,
        LocationType::Medium => 3,
        LocationType::Small => 2,
    };
    let max_j = match locations[j].loc_type {
        LocationType::Large => 5,
        LocationType::Medium => 3,
        LocationType::Small => 2,
    };
    if conn[i] >= max_i || conn[j] >= max_j {
        return false;
    }
    let road = create_road_segment(
        locations[i].x, locations[i].y,
        locations[j].x, locations[j].y,
        heights,
    );
    roads.push(road);
    conn[i] += 1;
    conn[j] += 1;
    true
}

/// 分层生成道路网络：
///   Level 1（主干道）— Large 城市之间用 MST 连接 + 额外 1~2 条冗余环路
///   Level 2（次级路）— Medium 小镇连接到最近且有容量的 Large 城市
///   Level 3（支路）  — Small 村庄连接到最近且有容量的 Medium/Large
///   兜底              — 给仍有孤立点的节点连到最近邻居
fn generate_roads(locations: &[LocationData], heights: &[f64]) -> Vec<RoadData> {
    let n = locations.len();
    if n <= 1 {
        return Vec::new();
    }

    let mut roads = Vec::new();
    let mut conn = vec![0u32; n];

    // 预处理所有欧几里得距离
    let mut dist = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        for j in 0..n {
            let dx = locations[i].x - locations[j].x;
            let dy = locations[i].y - locations[j].y;
            dist[i][j] = (dx * dx + dy * dy).sqrt();
        }
    }

    // 按类型分组索引
    let large: Vec<usize> = (0..n)
        .filter(|i| matches!(locations[*i].loc_type, LocationType::Large))
        .collect();
    let medium: Vec<usize> = (0..n)
        .filter(|i| matches!(locations[*i].loc_type, LocationType::Medium))
        .collect();
    let small: Vec<usize> = (0..n)
        .filter(|i| matches!(locations[*i].loc_type, LocationType::Small))
        .collect();

    // ── Level 1：主干道（Large 城市间 MST + 冗余环路） ──────
    if large.len() >= 2 {
        let mut edges: Vec<(f64, usize, usize)> = Vec::new();
        for a in 0..large.len() {
            for b in (a + 1)..large.len() {
                edges.push((dist[large[a]][large[b]], large[a], large[b]));
            }
        }
        edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // MST 确保所有 Large 城市连通
        let mut uf = UnionFind::new(n);
        let target = large.len() - 1;
        let mut added = 0;
        for &(_d, i, j) in &edges {
            if added >= target {
                break;
            }
            if uf.union(i, j) {
                if try_add_road(&mut roads, &mut conn, locations, heights, i, j) {
                    added += 1;
                }
            }
        }

        // 补充 1~2 条最短冗余边，形成环路
        let extra_target = (large.len() / 4).max(1).min(2);
        let mut extra = 0;
        for &(_d, i, j) in &edges {
            if extra >= extra_target {
                break;
            }
            if try_add_road(&mut roads, &mut conn, locations, heights, i, j) {
                extra += 1;
            }
        }
    }

    // 收集主干道路网，供后续接入
    let level1_roads: Vec<RoadData> = roads.clone();

    // ── Level 2：次级路（Medium → 接入主干道，兜底 Large） ──
    for &m in &medium {
        if conn[m] >= 3 {
            continue;
        }

        // 优先接入主干道（最近点）
        if let Some((nx, ny)) = nearest_point_on_roads(&level1_roads, locations[m].x, locations[m].y) {
            let road = create_road_segment(
                locations[m].x, locations[m].y,
                nx, ny,
                heights,
            );
            roads.push(road);
            conn[m] += 1;
            continue;
        }

        // 回退：连到最近且有容量的 Large
        let mut best: Option<usize> = None;
        let mut best_d = f64::MAX;
        for &l in &large {
            if conn[l] >= 5 {
                continue;
            }
            let d = dist[m][l];
            if d < best_d {
                best_d = d;
                best = Some(l);
            }
        }
        if let Some(l) = best {
            try_add_road(&mut roads, &mut conn, locations, heights, m, l);
        }
    }

    // ── Level 3：支路（Small → 接入主干道，兜底 Medium/Large） ──
    for &s in &small {
        if conn[s] >= 2 {
            continue;
        }

        // 优先接入主干道
        if let Some((nx, ny)) = nearest_point_on_roads(&level1_roads, locations[s].x, locations[s].y) {
            let road = create_road_segment(
                locations[s].x, locations[s].y,
                nx, ny,
                heights,
            );
            roads.push(road);
            conn[s] += 1;
            continue;
        }

        // 优先找最近且有容量的 Medium
        let mut best: Option<usize> = None;
        let mut best_d = f64::MAX;
        for &m in &medium {
            if conn[m] >= 3 {
                continue;
            }
            let d = dist[s][m];
            if d < best_d {
                best_d = d;
                best = Some(m);
            }
        }
        // Medium 不可用时找 Large
        if best.is_none() {
            best_d = f64::MAX;
            for &l in &large {
                if conn[l] >= 5 {
                    continue;
                }
                let d = dist[s][l];
                if d < best_d {
                    best_d = d;
                    best = Some(l);
                }
            }
        }
        if let Some(target) = best {
            try_add_road(&mut roads, &mut conn, locations, heights, s, target);
        }
    }

    // ── 兜底：确保没有孤立节点（优先接入主干道） ──────
    for i in 0..n {
        if conn[i] > 0 {
            continue;
        }

        // 优先接入主干道
        if let Some((nx, ny)) = nearest_point_on_roads(&level1_roads, locations[i].x, locations[i].y) {
            let road = create_road_segment(
                locations[i].x, locations[i].y,
                nx, ny,
                heights,
            );
            roads.push(road);
            conn[i] += 1;
            continue;
        }

        // 找最近且有容量的节点
        let mut best: Option<usize> = None;
        let mut best_d = f64::MAX;
        for j in 0..n {
            if i == j {
                continue;
            }
            let max_j = match locations[j].loc_type {
                LocationType::Large => 5,
                LocationType::Medium => 3,
                LocationType::Small => 2,
            };
            if conn[j] >= max_j {
                continue;
            }
            let d = dist[i][j];
            if d < best_d {
                best_d = d;
                best = Some(j);
            }
        }
        if let Some(j) = best {
            try_add_road(&mut roads, &mut conn, locations, heights, i, j);
        } else {
            // 终极兜底：忽略容量限制
            let mut j = if i == 0 { 1 } else { 0 };
            let mut best_d = dist[i][j];
            for k in 0..n {
                if k == i {
                    continue;
                }
                if dist[i][k] < best_d {
                    best_d = dist[i][k];
                    j = k;
                }
            }
            let road = create_road_segment(
                locations[i].x, locations[i].y,
                locations[j].x, locations[j].y,
                heights,
            );
            roads.push(road);
            conn[i] += 1;
        }
    }

    roads
}

// ── A* 寻路 ──────────────────────────────────────

/// A* 节点
#[derive(Clone)]
struct AStarNode {
    f_score: f64,
    pos: (u32, u32),
}

impl Eq for AStarNode {}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// 在高度图上执行 A* 寻路，返回像素坐标路径
/// 代价综合考虑距离和高度变化，使道路偏好平坦地形（少上下坡）
fn a_star_path(
    heights: &[f64],
    start: (u32, u32),
    goal: (u32, u32),
) -> Option<Vec<(u32, u32)>> {
    let w = MAP_WIDTH as usize;
    let h = MAP_HEIGHT as usize;

    let mut g = vec![f64::MAX; w * h];
    let mut parent = vec![(u32::MAX, u32::MAX); w * h];
    let mut open = BinaryHeap::new();

    let h_start = ((start.0 as f64 - goal.0 as f64).powi(2)
        + (start.1 as f64 - goal.1 as f64).powi(2))
    .sqrt();
    g[(start.1 as usize) * w + start.0 as usize] = 0.0;
    open.push(AStarNode { f_score: h_start, pos: start });

    const DX: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
    const DY: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    const BASE_COST: [f64; 8] = [1.414, 1.0, 1.414, 1.0, 1.0, 1.414, 1.0, 1.414];

    let goal_f = (goal.0 as f64, goal.1 as f64);

    while let Some(node) = open.pop() {
        if node.pos == goal {
            let mut path = Vec::new();
            let mut cur = node.pos;
            while cur != start {
                path.push(cur);
                let idx = cur.1 as usize * w + cur.0 as usize;
                cur = parent[idx];
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        let cur_idx = node.pos.1 as usize * w + node.pos.0 as usize;
        let cur_g = g[cur_idx];
        let cur_h = heights[cur_idx];

        for i in 0..8 {
            let nx = node.pos.0 as i32 + DX[i];
            let ny = node.pos.1 as i32 + DY[i];

            if nx < 0 || nx >= MAP_WIDTH as i32 || ny < 0 || ny >= MAP_HEIGHT as i32 {
                continue;
            }

            let nu = nx as u32;
            let nv = ny as u32;
            let nidx = nv as usize * w + nu as usize;
            let nh = heights[nidx];

            // 代价 = 距离 + 高度变化惩罚（尽量少上下坡）
            let dh = (nh - cur_h).abs();
            let step_cost = BASE_COST[i] + dh * SLOPE_PENALTY;
            let tentative_g = cur_g + step_cost;

            if tentative_g < g[nidx] {
                g[nidx] = tentative_g;
                parent[nidx] = node.pos;

                let heuristic = ((nu as f64 - goal_f.0).powi(2)
                    + (nv as f64 - goal_f.1).powi(2))
                .sqrt();
                open.push(AStarNode { f_score: tentative_g + heuristic, pos: (nu, nv) });
            }
        }
    }

    None
}

// ── RDP 路径简化 ──────────────────────────────────

/// 点到线段的垂直距离
fn perp_distance(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (px, py) = p;
    let (ax, ay) = a;
    let (bx, by) = b;
    let dx = bx - ax;
    let dy = by - ay;
    let len_sq = dx * dx + dy * dy;

    if len_sq < 1e-10 {
        return ((px - ax).powi(2) + (py - ay).powi(2)).sqrt();
    }

    let t = ((px - ax) * dx + (py - ay) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);

    let cx = ax + t * dx;
    let cy = ay + t * dy;
    ((px - cx).powi(2) + (py - cy).powi(2)).sqrt()
}

/// Ramer–Douglas–Peucker 路径简化
fn rdp_simplify(points: &[(f64, f64)], epsilon: f64) -> Vec<(f64, f64)> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let mut max_d = 0.0;
    let mut max_i = 0;
    let first = points[0];
    let last = points[points.len() - 1];

    for i in 1..points.len() - 1 {
        let d = perp_distance(points[i], first, last);
        if d > max_d {
            max_d = d;
            max_i = i;
        }
    }

    if max_d > epsilon {
        let mut left = rdp_simplify(&points[..=max_i], epsilon);
        let right = rdp_simplify(&points[max_i..], epsilon);
        left.pop(); // 去除重复端点
        left.extend(right);
        left
    } else {
        vec![points[0], points[points.len() - 1]]
    }
}

// ── 道路几何工具 ──────────────────────────────────

/// 在道路折线集合中找出离 (px,py) 最近的点
fn nearest_point_on_roads(roads: &[RoadData], px: f64, py: f64) -> Option<(f64, f64)> {
    let mut best: Option<(f64, f64)> = None;
    let mut best_d2 = f64::MAX;

    for road in roads {
        let pts = &road.points;
        // road.points 是扁平数组 [x1,y1,x2,y2,...]
        for i in (0..pts.len().saturating_sub(3)).step_by(2) {
            let ax = pts[i];
            let ay = pts[i + 1];
            let bx = pts[i + 2];
            let by = pts[i + 3];

            let seg_dx = bx - ax;
            let seg_dy = by - ay;
            let len_sq = seg_dx * seg_dx + seg_dy * seg_dy;

            let (nx, ny) = if len_sq < 1e-10 {
                (ax, ay)
            } else {
                let t = ((px - ax) * seg_dx + (py - ay) * seg_dy) / len_sq;
                let t = t.clamp(0.0, 1.0);
                (ax + t * seg_dx, ay + t * seg_dy)
            };

            let d2 = (px - nx).powi(2) + (py - ny).powi(2);
            if d2 < best_d2 {
                best_d2 = d2;
                best = Some((nx, ny));
            }
        }
    }

    best
}

// ── 创建单条道路 ──────────────────────────────────

/// 基于地形的 A* 寻路创建两点间的道路折线（坐标参数）
fn create_road_segment(
    from_x: f64, from_y: f64,
    to_x: f64, to_y: f64,
    heights: &[f64],
) -> RoadData {
    let start_px = world_to_pixel(from_x, from_y);
    let end_px = world_to_pixel(to_x, to_y);

    if let Some(pixel_path) = a_star_path(heights, start_px, end_px) {
        let world_path: Vec<(f64, f64)> = pixel_path
            .iter()
            .map(|&(px, py)| pixel_to_world(px, py))
            .collect();

        let simplified = rdp_simplify(&world_path, 3.0);

        // 确保首尾点精确等于端点坐标（补偿像素取整误差）
        let mut points = Vec::with_capacity(simplified.len() * 2 + 4);
        points.push((from_x * 10.0).round() / 10.0);
        points.push((from_y * 10.0).round() / 10.0);
        for &(x, y) in &simplified[1..simplified.len().saturating_sub(1)] {
            points.push((x * 10.0).round() / 10.0);
            points.push((y * 10.0).round() / 10.0);
        }
        points.push((to_x * 10.0).round() / 10.0);
        points.push((to_y * 10.0).round() / 10.0);

        RoadData { points }
    } else {
        // 回退：直接直线 + 控制点
        let mid_x = (from_x + to_x) / 2.0;
        let mid_y = (from_y + to_y) / 2.0;
        let dx = to_x - from_x;
        let dy = to_y - from_y;
        let len = (dx * dx + dy * dy).sqrt().max(1.0);
        let perp_x = -dy / len;
        let perp_y = dx / len;
        let offset = len * 0.15;
        let cp1x = mid_x + perp_x * offset * 0.8;
        let cp1y = mid_y + perp_y * offset * 0.8;
        let cp2x = mid_x - perp_x * offset * 0.5;
        let cp2y = mid_y - perp_y * offset * 0.5;

        RoadData {
            points: vec![
                (from_x * 10.0).round() / 10.0,
                (from_y * 10.0).round() / 10.0,
                (cp1x * 10.0).round() / 10.0,
                (cp1y * 10.0).round() / 10.0,
                (cp2x * 10.0).round() / 10.0,
                (cp2y * 10.0).round() / 10.0,
                (to_x * 10.0).round() / 10.0,
                (to_y * 10.0).round() / 10.0,
            ],
        }
    }
}
