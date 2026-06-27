mod heightmap;
mod settlements;
mod network;
mod roads;
mod boundary;
mod habitability;

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

// ── 地点生成参数 ──────────────────────────────────
pub(crate) const CITY_COUNT: usize = 3;
pub(crate) const TOWN_COUNT: usize = 20;
pub(crate) const TOTAL_TARGET: usize = 180;

// ── 地点间距约束（世界单位）─────────────────────────
pub(crate) const DIST_CITY_CITY: f64 = 240.0;
pub(crate) const DIST_CITY_TOWN: f64 = 90.0;
pub(crate) const DIST_TOWN_TOWN: f64 = 90.0;
pub(crate) const DIST_TOWN_VILLAGE: f64 = 30.0;
pub(crate) const DIST_VILLAGE_VILLAGE: f64 = 30.0;
pub(crate) const DIST_VILLAGE_CITY: f64 = 30.0;

// ── 吸引力参数（采样偏向聚落中心，无硬性最大距离）─────
pub(crate) const ATTRACT_NEAR_PROB: f64 = 0.85;
pub(crate) const ATTRACT_CITY_TOWN_FALLOFF: f64 = 300.0;
pub(crate) const ATTRACT_TOWN_VILLAGE_FALLOFF: f64 = 120.0;
pub(crate) const ATTRACT_CITY_VILLAGE_FALLOFF: f64 = 350.0;

// ── 地形过滤阈值 ──────────────────────────────────
#[allow(dead_code)]
pub(crate) const FLAT_THRESHOLD: f64 = 0.10;

// ── 边界生成参数 ──────────────────────────────────
pub(crate) const GRID_N: u32 = 12;

// ── 宜居度参数 ────────────────────────────────────
pub(crate) const SLOPE_STEEP: f64 = 0.025;
pub(crate) const MAX_RAD_PX: f64 = 64.0;
pub(crate) const W_ALT: f64 = 0.30;
pub(crate) const W_SLP: f64 = 0.20;
pub(crate) const W_RAD: f64 = 0.50;

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
    pub habitability: Vec<f32>,   // 512×512, 0..1 宜居度
    pub width: u32,
    pub height: u32,
    pub boundary: Vec<f64>,       // 扁平 [x0,y0,x1,y1,...] 世界坐标闭合环
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

    // 1a. 生成边界 (纯形状约束, 不依赖高度)
    let boundary = boundary::generate_boundary(seed);

    // 1b. 预计算全图宜居度
    let habitability = habitability::compute_habitability(&heightmap);

    // 1c. 构建宜居度加权 CDF (边界内 + 高宜居度候选像素)，供三层采样复用
    let hab_cdf = settlements::build_hab_cdf(&boundary, &habitability);

    // 2. 分层生成位置：城市 → 小镇 → 村庄 (传入边界 + 宜居度 + CDF)
    let cities = settlements::generate_cities(&heightmap, &boundary, &habitability, &hab_cdf, seed);
    let towns = settlements::generate_towns(&heightmap, &boundary, &habitability, &hab_cdf, seed, &cities);
    let villages = settlements::generate_villages(&heightmap, &boundary, &habitability, &hab_cdf, seed, &cities, &towns);

    // 2. 合并所有位置并构建 LocationData（类型在生成时已确定）
    let cap = cities.len() + towns.len() + villages.len();
    let mut positions: Vec<(f64, f64)> = Vec::with_capacity(cap);
    let mut locations: Vec<LocationData> = Vec::with_capacity(cap);
    let mut id = 1usize;

    for &(x, y) in &cities {
        positions.push((x, y));
        locations.push(LocationData {
            id: format!("loc_{}", id),
            name: settlements::pick_name(&LocationType::Large, &mut rng),
            x, y,
            loc_type: LocationType::Large,
        });
        id += 1;
    }
    for &(x, y) in &towns {
        positions.push((x, y));
        locations.push(LocationData {
            id: format!("loc_{}", id),
            name: settlements::pick_name(&LocationType::Medium, &mut rng),
            x, y,
            loc_type: LocationType::Medium,
        });
        id += 1;
    }
    for &(x, y) in &villages {
        positions.push((x, y));
        locations.push(LocationData {
            id: format!("loc_{}", id),
            name: settlements::pick_name(&LocationType::Small, &mut rng),
            x, y,
            loc_type: LocationType::Small,
        });
        id += 1;
    }

    // 3. 日志
    log::info!(
        "Map [seed={}]: {} locations (Large={}, Medium={}, Small={})",
        seed, locations.len(), cities.len(), towns.len(), villages.len(),
    );

    // 4. Delaunay 三角剖分 → 候选边
    let mut edges = network::build_delaunay_edges(&positions);

    // 5. 过滤冗余边：删除存在替代路径且绕路不超过 30% 的边
    let before = edges.len();
    edges = network::filter_redundant_edges(&edges, &positions);
    log::info!("Map [seed={}]: {} redundant edges removed ({} → {})",
        seed, before - edges.len(), before, edges.len());

    // 6. 连通性兜底（冗余边删除后可能产生孤立分量）
    edges = network::ensure_connectivity(&edges, &positions);

    // 7. 基于边集生成道路 (约束在边界内)
    let roads = roads::generate_roads_from_edges(&locations, &edges, &heightmap, &boundary.mask);
    log::info!("Map [seed={}]: {} roads generated", seed, roads.len());

    // 8. 扁平化边界多边形 (与 RoadData.points 格式一致)
    let boundary_poly = boundary.polygon.clone();

    MapData {
        heightmap: heightmap.into_iter().map(|v| v as f32).collect(),
        habitability,
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
        boundary: boundary_poly,
        locations,
        roads,
    }
}
