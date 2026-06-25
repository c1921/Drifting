mod heightmap;
mod settlements;
mod network;
mod roads;

use rand::Rng;
use serde::Serialize;

// ── 地图常量 ──────────────────────────────────────
pub const MAP_WIDTH: u32 = 512;
pub const MAP_HEIGHT: u32 = 512;
pub const MAP_SCALE: f64 = 0.008;
pub const OCTAVES: usize = 4;
pub const PERSISTENCE: f64 = 0.5;
pub const LACUNARITY: f64 = 2.0;

pub(crate) const SLOPE_PENALTY: f64 = 300.0;
pub(crate) const WORLD_MIN: f64 = -800.0;
pub(crate) const WORLD_MAX: f64 = 800.0;

// ── 数据结构 ──────────────────────────────────────

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum LocationType {
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "small")]
    Small,
}

#[derive(Debug, Clone, Serialize)]
pub struct LocationData {
    pub id: String,
    pub name: String,
    pub x: f64,
    pub y: f64,
    #[serde(rename = "type")]
    pub loc_type: LocationType,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoadData {
    pub points: Vec<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MapData {
    pub heightmap: Vec<f32>,
    pub width: u32,
    pub height: u32,
    pub locations: Vec<LocationData>,
    pub roads: Vec<RoadData>,
}

// ── 名称池（按地点规模分级）────────────────────

pub(crate) const LARGE_NAMES: &[&str] = &[
    "Aetheris", "Bastion", "Celestia", "Dragonport", "Everbright",
    "Goldspire", "Highreach", "Ironhold", "King's Landing", "Luminara",
    "Meridian", "Northgate", "Obsidian", "Palisade", "Queensford",
];

pub(crate) const MEDIUM_NAMES: &[&str] = &[
    "Ashford", "Briarwood", "Copperford", "Dawnhold", "Eastwatch",
    "Fairhaven", "Glenwood", "Hollowshire", "Ivorygate", "Jadeport",
    "Kingsford", "Lunaris", "Maplecrest", "Northbrook", "Oakhaven",
    "Pinehaven", "Riverton", "Silverbrook", "Starlight", "Stonebridge",
    "Thornwall", "Willowdale", "Westmarch", "Wyncrest", "Millbrook",
];

pub(crate) const SMALL_NAMES: &[&str] = &[
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

    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed([seed as u8; 32]);

    let heightmap = heightmap::generate_heightmap(seed);

    // 1. 生成位置（h<0.10 低海拔平原，无类型标记）
    let positions = settlements::generate_positions(&heightmap, seed);

    // 2. Delaunay 三角剖分 → 候选边
    let mut edges = network::build_delaunay_edges(&positions);

    // 3. 过滤长边（中位数 × 3）
    edges = network::filter_long_edges(&edges, &positions);

    // 4. 连通性兜底
    edges = network::ensure_connectivity(&edges, &positions);

    // 5. 图中心性评分
    let scores = network::compute_settlement_scores(&positions, &edges);

    // 6. 百分位排名 → 类型分配（0=Small, 1=Medium, 2=Large）
    let type_classes = settlements::classify_by_central_place(&positions, &scores);

    // 7. 构建 LocationData
    let locations: Vec<LocationData> = positions.iter().enumerate()
        .map(|(i, &(x, y))| {
            let loc_type = match type_classes[i] {
                2 => LocationType::Large,
                1 => LocationType::Medium,
                _ => LocationType::Small,
            };
            let name = settlements::pick_name(&loc_type, &mut rng);
            LocationData {
                id: format!("loc_{}", i + 1),
                name,
                x,
                y,
                loc_type,
            }
        })
        .collect();

    // 日志
    let large_count = locations.iter().filter(|l| matches!(l.loc_type, LocationType::Large)).count();
    let medium_count = locations.iter().filter(|l| matches!(l.loc_type, LocationType::Medium)).count();
    let small_count = locations.iter().filter(|l| matches!(l.loc_type, LocationType::Small)).count();
    log::info!(
        "Map [seed={}]: {} locations (Large={}, Medium={}, Small={})",
        seed, locations.len(), large_count, medium_count, small_count,
    );

    // 7. 过滤冗余边：删除存在替代路径且绕路不超过 30% 的边
    let before = edges.len();
    edges = network::filter_redundant_edges(&edges, &positions);
    log::info!("Map [seed={}]: {} redundant edges removed ({} → {})",
        seed, before - edges.len(), before, edges.len());

    // 8. 再次连通性兜底（冗余边删除后可能产生孤立分量）
    edges = network::ensure_connectivity(&edges, &positions);

    // 9. 基于边集生成道路
    let roads = roads::generate_roads_from_edges(&locations, &edges, &heightmap);
    log::info!("Map [seed={}]: {} roads generated", seed, roads.len());

    MapData {
        heightmap: heightmap.into_iter().map(|v| v as f32).collect(),
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
        locations,
        roads,
    }
}
